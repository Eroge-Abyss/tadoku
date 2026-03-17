use crate::{AppState, prelude::Result, services::stores::settings::PlaytimeMode, util};
use anyhow::Context;
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info, warn};
use serde::Deserialize;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager};
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

pub struct ExStaticPlaytime;

impl ExStaticPlaytime {
    fn process_input(input: String) -> Result<ExStaticData> {
        debug!("Processing input: {}", input);
        serde_json::from_str(&input).context("Failed to deserialize exstatic input")
    }

    fn handle(app_handle: &AppHandle, data: ExStaticData) -> Result<()> {
        debug!("Handling ExStatic data: {:?}", data);

        let binding = app_handle.state::<Mutex<AppState>>();
        let mut state = binding
            .lock()
            .map_err(|e| anyhow::anyhow!("Error acquiring mutex lock: {}", e))?;

        if !matches!(&state.settings.playtime_mode, PlaytimeMode::ExStatic) {
            debug!("PlaytimeMode is not ExStatic, ignoring data");
            return Ok(());
        }

        if let (Some(game), Some(pid)) = (
            state.game.as_mut(),
            util::get_pid_from_process_path(&data.process_path),
        ) {
            if pid.as_u32() == game.pid {
                let time = data.time.round() as u64;
                info!("Updating playtime for game {} by {} seconds", game.id, time);
                game.current_playtime += time;
                util::flush_playtime(app_handle, &game.id, time)?;

                // Update chars_read if provided by exSTATic
                if let Some(chars_read) = data.chars_read {
                    debug!("Updating chars_read for game {} to {}", game.id, chars_read);
                    util::flush_chars_read(app_handle, &game.id, chars_read)?;
                    if let Err(e) = app_handle.emit("chars_read_updated", chars_read) {
                        error!("Error emitting chars_read_updated event: {}", e);
                    }
                }
            } else {
                warn!(
                    "PID mismatch: data PID {} != game PID {}",
                    pid.as_u32(),
                    game.pid
                );
            }
        } else {
            debug!("No game running or PID not found, ignoring data");
        }

        Ok(())
    }

    pub fn spawn(app_handle: &AppHandle, token: tokio_util::sync::CancellationToken) {
        info!("Spawning ExStatic playtime tracking task");
        let app_handle = app_handle.clone();
        tauri::async_runtime::spawn(async move {
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
                        let app_handle = app_handle.clone();
                        let conn_token = token.clone();
                        connection_handlers.spawn(Self::handle_connection(app_handle, stream, conn_token));
                    }
                }
            }

            info!(
                "Waiting for {} active connection(s) to shut down...",
                connection_handlers.len()
            );
            connection_handlers.shutdown().await;
            info!("All connections closed. ExStatic server shut down completely.");
        });
    }

    async fn handle_connection(
        app_handle: AppHandle,
        stream: TcpStream,
        conn_token: CancellationToken,
    ) {
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
                                match Self::process_input(msg_text.to_string()) {
                                    Ok(data) => {
                                        if let Err(e) = Self::handle(&app_handle, data) {
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
}
