use crate::{
    prelude::Result,
    services::{
        playtime::PlaytimeService,
        state::ManagedState,
        stores::{games::GamesStore, settings::PlaytimeMode},
        system::SystemService,
    },
};
use anyhow::Context;
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info, warn};
use serde::Deserialize;
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tokio_util::sync::CancellationToken;

const SERVER_ADDRESS: &str = "127.0.0.1:6969";

#[derive(Deserialize, Debug)]
struct ExStaticData {
    time: f64,
    process_path: String,
    chars_read: Option<u64>,
}

pub struct ExStaticPlaytime {
    app_handle: AppHandle,
    playtime_service: Arc<PlaytimeService>,
}

impl ExStaticPlaytime {
    pub fn new(app_handle: AppHandle, store: GamesStore) -> Arc<Self> {
        Arc::new(Self {
            playtime_service: Arc::new(PlaytimeService::new(app_handle.clone(), store)),
            app_handle,
        })
    }

    pub fn spawn(app_handle: &AppHandle, token: CancellationToken) -> Result<()> {
        let store = GamesStore::new(app_handle)?;
        let server = Self::new(app_handle.clone(), store);
        server.start(token);
        Ok(())
    }

    pub fn start(self: &Arc<Self>, token: CancellationToken) {
        info!("Spawning ExStatic playtime tracking task");
        let server = Arc::clone(self);
        tauri::async_runtime::spawn(async move {
            server.listen(token).await;
        });
    }

    async fn listen(self: Arc<Self>, token: CancellationToken) {
        let listener = match TcpListener::bind(SERVER_ADDRESS).await.context(format!(
            "Failed to bind WebSocket server to {}",
            SERVER_ADDRESS
        )) {
            Ok(l) => {
                info!("WebSocket server listening on {}", SERVER_ADDRESS);
                l
            }
            Err(e) => {
                error!("{:?}", e);
                return;
            }
        };

        let mut connection_handlers = tokio::task::JoinSet::new();

        loop {
            tokio::select! {
                _ = token.cancelled() => {
                    info!("Shutdown signal received, closing WebSocket server.");
                    break;
                }
                Ok((stream, _)) = listener.accept() => {
                    let server = Arc::clone(&self);
                    let conn_token = token.clone();
                    connection_handlers.spawn(async move {
                        server.handle_connection(stream, conn_token).await;
                    });
                }
            }
        }

        info!(
            "Waiting for {} active connection(s) to shut down...",
            connection_handlers.len()
        );
        connection_handlers.shutdown().await;
        info!("All connections closed. ExStatic server shut down completely.");
    }

    async fn handle_connection(&self, stream: TcpStream, conn_token: CancellationToken) {
        let ws_stream = match accept_async(stream).await {
            Ok(ws) => {
                info!("New WebSocket connection established!");
                ws
            }
            Err(e) => {
                error!("Error during WebSocket handshake: {}", e);
                return;
            }
        };

        let (mut write, mut read) = ws_stream.split();
        loop {
            tokio::select! {
                _ = conn_token.cancelled() => {
                    info!("Closing WebSocket connection due to shutdown signal.");
                    let _ = write.send(tokio_tungstenite::tungstenite::Message::Close(None)).await;
                    break;
                }
                message = read.next() => {
                    match message {
                        Some(Ok(msg)) => {
                            if msg.is_text() {
                                let msg_text = msg.into_text().expect("already checked for text");
                                match Self::process_input(&msg_text) {
                                    Ok(data) => {
                                        if let Err(e) = self.handle(data) {
                                            error!("Error handling ExStatic data: {}", e);
                                        }
                                    }
                                    Err(e) => error!("Error processing input: {}", e),
                                }
                            }
                        }
                        Some(Err(e)) => {
                            error!("Error processing message: {}", e);
                            break;
                        }
                        None => break, // Connection closed by client
                    }
                }
            }
        }
        info!("WebSocket connection closed.");
    }

    fn process_input(input: &str) -> Result<ExStaticData> {
        debug!("Processing input: {}", input);
        serde_json::from_str(input).context("Failed to deserialize exstatic input")
    }

    fn handle(&self, data: ExStaticData) -> Result<()> {
        debug!("Handling ExStatic data: {:?}", data);

        let (is_exstatic, game_pid, game_id) = {
            let managed = self.app_handle.state::<ManagedState>();
            let state = managed
                .lock()
                .map_err(|e| anyhow::anyhow!("Error acquiring mutex lock: {}", e))?;

            (
                matches!(&state.settings.playtime_mode, PlaytimeMode::ExStatic),
                state.game.as_ref().map(|g| g.pid),
                state.game.as_ref().map(|g| g.id.clone()),
            )
        };

        if !is_exstatic {
            debug!("PlaytimeMode is not ExStatic, ignoring data");
            return Ok(());
        }

        let Some(active_pid) = game_pid else {
            debug!("No game running, ignoring data");
            return Ok(());
        };

        let Some(matched_pid) = SystemService::get_pid_from_process_path(&data.process_path) else {
            debug!("Process PID not found, ignoring data");
            return Ok(());
        };

        if matched_pid.as_u32() != active_pid {
            warn!(
                "PID mismatch: data PID {} != game PID {}",
                matched_pid.as_u32(),
                active_pid
            );
            return Ok(());
        }

        let time = data.time.round() as u64;
        if time > 0 {
            self.playtime_service.record_time(time);
        }

        if let Some(chars_read) = data.chars_read {
            if let Some(id) = game_id {
                debug!("Updating chars_read for game {} to {}", id, chars_read);
            }
            self.playtime_service.record_chars_read(chars_read);
        }

        Ok(())
    }
}
