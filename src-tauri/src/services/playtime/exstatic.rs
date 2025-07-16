use crate::{prelude::Result, services::stores::settings::PlaytimeMode, util, AppState};
use futures_util::StreamExt;
use serde::Deserialize;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

#[derive(Deserialize, Debug)]
struct ExStaticData {
    time: f64,
    process_path: String,
}

pub struct ExStaticPlaytime;

impl ExStaticPlaytime {
    fn process_input(input: String) -> Result<ExStaticData> {
        let v: ExStaticData = serde_json::from_str(&input)?;
        Ok(v)
    }

    fn handle(app_handle: &AppHandle, data: ExStaticData) -> Result<()> {
        // Check if a game is running
        let binding = app_handle.state::<Mutex<AppState>>();
        let mut state = binding
            .lock()
            .map_err(|_| "Error acquiring mutex lock".to_string())?;

        // For simplicity, if setting is not enabled don't process
        if matches!(&state.config.playtime_mode, PlaytimeMode::ExStatic) {
            // If yes then update time as needed
            // If not then ignore
            if let (Some(game), Some(pid)) = (
                state.game.as_mut(),
                util::get_pid_from_process_path(&data.process_path),
            ) {
                if pid.as_u32() == game.pid {
                    let time = data.time.round() as u64;
                    game.current_playtime += time;
                    util::flush_playtime(app_handle, &game.id, time)?;
                }
            }
        }

        Ok(())
    }

    pub fn spawn(app_handle: &AppHandle) {
        let app_handle = app_handle.clone();
        tauri::async_runtime::spawn(async move {
            let addr = "127.0.0.1:6969";
            let listener = match TcpListener::bind(addr).await {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("Failed to bind WebSocket server to {}: {}", addr, e);
                    return;
                }
            };
            println!("WebSocket server listening on {}", addr);

            while let Ok((stream, _)) = listener.accept().await {
                let app_handle = app_handle.clone(); // Clone for each connection
                tauri::async_runtime::spawn(async move {
                    let ws_stream = match accept_async(stream).await {
                        Ok(ws) => ws,
                        Err(e) => {
                            eprintln!("Error during WebSocket handshake: {}", e);
                            return;
                        }
                    };
                    println!("New WebSocket connection established!");

                    let (_, mut read) = ws_stream.split();
                    while let Some(message) = read.next().await {
                        match message {
                            Ok(msg) => {
                                if msg.is_text() {
                                    let msg = msg
                                        .into_text()
                                        .expect("Shouldn't happen, already made an if check");

                                    if let Ok(data) = Self::process_input(msg) {
                                        let _ = Self::handle(&app_handle, data).inspect_err(|e| {
                                            eprintln!("{e}");
                                        });
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Error processing message: {}", e);
                                break;
                            }
                        }
                    }

                    println!("WebSocket connection closed.");
                });
            }
        });
    }
}
