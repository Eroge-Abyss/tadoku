use super::super::stores::games::GamesStore;
#[cfg(windows)]
use crate::util::flush_playtime;
use crate::{services::stores::settings::PlaytimeMode, AppState};
use log::{debug, error, info, warn};
use serde_json::json;
use std::{sync::Mutex, thread, time::Duration};
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};
use tauri::{AppHandle, Emitter, Manager};

pub struct ClassicPlaytime;

impl ClassicPlaytime {
    pub fn spawn(app_handle: &AppHandle) {
        info!("Spawning classic playtime tracking task");
        let app_handle = app_handle.clone();
        tauri::async_runtime::spawn(async move {
            debug!("Classic playtime task started");
            let mut system = System::new_with_specifics(
                RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
            );

            loop {
                let (pid, game_id, current_playtime, playtime_mode) = {
                    let state = app_handle.state::<Mutex<AppState>>();
                    let mut state = match state.lock() {
                        Ok(s) => s,
                        Err(e) => {
                            error!("Failed to acquire app state mutex lock: {}", e);
                            return;
                        }
                    };

                    let game_state = match state.game.as_mut().ok_or("Couldn't find the game") {
                        Ok(g) => g,
                        Err(e) => {
                            warn!("No active game found: {}", e);
                            return;
                        }
                    };

                    debug!(
                        "Current game details - PID: {}, Game ID: {}",
                        game_state.pid, game_state.id
                    );

                    (
                        game_state.pid,
                        game_state.id.clone(),
                        game_state.current_playtime,
                        state.config.playtime_mode,
                    )
                };

                // Inline get_playtime logic
                system.refresh_processes_specifics(
                    ProcessesToUpdate::All,
                    true,
                    ProcessRefreshKind::everything(),
                );

                let process_playtime = system
                    .process(Pid::from(pid as usize))
                    .map(|p| p.run_time());

                match process_playtime {
                    Some(_) => {
                        if !matches!(playtime_mode, PlaytimeMode::Classic) {
                            debug!("Playtime mode is not Classic, skipping iteration");
                            thread::sleep(Duration::from_secs(1));
                            continue;
                        }

                        debug!("Playtime mode is Classic, proceeding with tracking");

                        #[cfg(windows)]
                        {
                            if let Ok(active_window) = x_win::get_active_window() {
                                if active_window.id == pid {
                                    debug!("Game is active, incrementing playtime");
                                    {
                                        let state = app_handle.state::<Mutex<AppState>>();
                                        let mut state = match state.lock() {
                                            Ok(s) => s,
                                            Err(e) => {
                                                error!(
                                                    "Failed to acquire app state mutex lock: {}",
                                                    e
                                                );
                                                return;
                                            }
                                        };
                                        let game_state = match state
                                            .game
                                            .as_mut()
                                            .ok_or("Couldn't find the game")
                                        {
                                            Ok(g) => g,
                                            Err(e) => {
                                                warn!("No active game found: {}", e);
                                                return;
                                            }
                                        };
                                        game_state.current_playtime += 1;
                                    }

                                    if current_playtime % 60 == 0 {
                                        flush_playtime(&app_handle, &game_id, 60).map_err(
                                            |_| "Error happened while updating playtime",
                                        )?;
                                    }

                                    // let store = GamesStore::new(&app_handle)
                                    //     .map_err(|_| "Error happened while accessing store")?;

                                    // store
                                    //     .update_playtime(&game_id, FLUSH_DURATION.into())
                                    debug!("Flushing playtime to store");
                                    if let Err(e) = util::flush_playtime(&app_handle, &game_id, 60)
                                    {
                                        error!("Error happened while updating playtime: {}", e);
                                    }

                                    if let Err(e) =
                                        app_handle.emit("playtime", current_playtime + 1)
                                    {
                                        error!("Error happened while emitting playtime: {}", e);
                                    }
                                } else {
                                    debug!("Game is not active, pausing playtime");
                                    if let Err(e) = app_handle.emit("playtime", "paused") {
                                        error!("Error happened while emitting playtime: {}", e);
                                    }
                                }
                            } else {
                                debug!("Failed to get active window");
                            }
                        }

                        #[cfg(not(windows))]
                        {
                            {
                                let state = app_handle.state::<Mutex<AppState>>();
                                let mut state = match state.lock() {
                                    Ok(s) => s,
                                    Err(e) => {
                                        error!("Failed to acquire app state mutex lock: {}", e);
                                        return;
                                    }
                                };
                                let game_state =
                                    match state.game.as_mut().ok_or("Couldn't find the game") {
                                        Ok(g) => g,
                                        Err(e) => {
                                            warn!("No active game found: {}", e);
                                            return;
                                        }
                                    };
                                game_state.current_playtime += 1;
                            }
                            if let Err(e) = app_handle.emit("playtime", current_playtime + 1) {
                                error!("Error happened while emitting playtime: {}", e);
                            }
                        }

                        debug!("Sleeping for 1 second");
                        thread::sleep(Duration::from_secs(1));
                    }
                    None => {
                        info!("Game process not found, stopping playtime tracking");
                        let store = match GamesStore::new(&app_handle) {
                            Ok(s) => s,
                            Err(e) => {
                                error!("Error happened while accessing store: {}", e);
                                return;
                            }
                        };

                        if let Err(e) = store.update_playtime(&game_id, current_playtime % 60) {
                            error!("Error happened while setting new playtime: {}", e);
                        }

                        if let Err(e) = store.update_last_played(&game_id) {
                            error!("Error happened while updating last played: {}", e);
                        }

                        let state = app_handle.state::<Mutex<AppState>>();
                        let mut state = match state.lock() {
                            Ok(s) => s,
                            Err(e) => {
                                error!("Failed to acquire app state mutex lock: {}", e);
                                return;
                            }
                        };
                        state.game = None;

                        if let Some(pres) = &mut state.presence {
                            if let Err(e) = pres.reset() {
                                error!("Error happened while clearing presence: {}", e);
                            }
                        }

                        if let Err(e) = app_handle.emit("current_game", json!(null)) {
                            error!("Error happened while emitting current_game event: {}", e);
                        }
                        break;
                    }
                }
            }
        });
    }
}
