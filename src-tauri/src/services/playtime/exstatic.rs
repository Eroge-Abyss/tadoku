use crate::{prelude::Result, services::stores::settings::PlaytimeMode, util, AppState};
use futures_util::StreamExt;
use log::{debug, error, info, warn};
use serde::Deserialize;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

const SERVER_ADDRESS: &str = "127.0.0.1:6969";

#[derive(Deserialize, Debug)]
struct ExStaticData {
    time: f64,
    process_path: String,
}

pub struct ExStaticPlaytime;

impl ExStaticPlaytime {
    fn process_input(input: String) -> Result<ExStaticData> {
        debug!("Processing input: {}", input);
        let v: ExStaticData = serde_json::from_str(&input).map_err(|e| {
            error!("Failed to deserialize input: {}", e);
            e.to_string()
        })?;
        Ok(v)
    }

    fn handle(app_handle: &AppHandle, data: ExStaticData) -> Result<()> {
        debug!("Handling ExStatic data: {:?}", data);

        let binding = app_handle.state::<Mutex<AppState>>();
        let mut state = binding.lock().map_err(|e| {
            error!("Error acquiring mutex lock: {}", e);
            "Error acquiring mutex lock".to_string()
        })?;

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

    pub fn spawn(app_handle: &AppHandle) {
        info!("Spawning ExStatic playtime tracking task");
        let app_handle = app_handle.clone();
        tauri::async_runtime::spawn(async move {
            info!("Binding WebSocket server to {}", SERVER_ADDRESS);
            let listener = match TcpListener::bind(SERVER_ADDRESS).await {
                Ok(l) => {
                    info!("Successfully bound WebSocket server to {}", SERVER_ADDRESS);
                    l
                }
                Err(e) => {
                    error!(
                        "Failed to bind WebSocket server to {}: {}",
                        SERVER_ADDRESS, e
                    );
                    return;
                }
            };
            info!("WebSocket server listening on {}", SERVER_ADDRESS);

            while let Ok((stream, _)) = listener.accept().await {
                let app_handle = app_handle.clone(); // Clone for each connection
                tauri::async_runtime::spawn(async move {
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

                    let (_, mut read) = ws_stream.split();
                    while let Some(message) = read.next().await {
                        match message {
                            Ok(msg) => {
                                if msg.is_text() {
                                    let msg = msg
                                        .into_text()
                                        .expect("Shouldn't happen, already made an if check");

                                    match Self::process_input(msg) {
                                        Ok(data) => {
                                            if let Err(e) = Self::handle(&app_handle, data) {
                                                error!("Error handling ExStatic data: {}", e);
                                            }
                                        }
                                        Err(e) => {
                                            error!("Error processing input: {}", e);
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Error processing message: {}", e);
                                break;
                            }
                        }
                    }

                    info!("WebSocket connection closed.");
                });
            }
        });
    }
}
