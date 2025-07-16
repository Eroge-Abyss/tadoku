use super::super::stores::games::GamesStore;
#[cfg(windows)]
use crate::util::flush_playtime;
use crate::{services::stores::settings::PlaytimeMode, AppState};
use serde_json::json;
use std::{sync::Mutex, thread, time::Duration};
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};
use tauri::{AppHandle, Emitter, Manager};

pub struct ClassicPlaytime;

impl ClassicPlaytime {
    pub fn spawn(app_handle: &AppHandle) {
        let app_handle = app_handle.clone();
        tauri::async_runtime::spawn(async move {
            let mut system = System::new_with_specifics(
                RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
            );

            loop {
                let (pid, game_id, current_playtime, playtime_mode) = {
                    let state = app_handle.state::<Mutex<AppState>>();
                    let mut state = state
                        .lock()
                        .map_err(|_| "Error happened while acquiring mutex lock")?;
                    let game_state = state.game.as_mut().ok_or("Couldn't find the game")?;

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
                    .map(|process| process.run_time());

                match process_playtime {
                    Some(_) => {
                        if !matches!(playtime_mode, PlaytimeMode::Classic) {
                            thread::sleep(Duration::from_secs(1));
                            continue;
                        }

                        #[cfg(windows)]
                        {
                            if let Ok(active_window) = x_win::get_active_window() {
                                if active_window.id == pid {
                                    // Should exec only if current window is active window
                                    {
                                        let state = app_handle.state::<Mutex<AppState>>();
                                        let mut state = state.lock().map_err(|_| {
                                            "Error happened while acquiring mutex lock"
                                        })?;
                                        let game_state =
                                            state.game.as_mut().ok_or("Couldn't find the game")?;
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
                                    //     .map_err(|_| "Error happened while setting new playtime")?;

                                    app_handle
                                        .emit("playtime", current_playtime + 1)
                                        .map_err(|_| "Error happened while emitting playtime")?;
                                } else {
                                    app_handle
                                        .emit("playtime", "paused")
                                        .map_err(|_| "Error happened while emitting playtime")?;
                                }
                            }
                        }

                        #[cfg(not(windows))]
                        {
                            {
                                let state = app_handle.state::<Mutex<AppState>>();
                                let mut state = state
                                    .lock()
                                    .map_err(|_| "Error happened while acquiring mutex lock")?;
                                let game_state =
                                    state.game.as_mut().ok_or("Couldn't find the game")?;
                                game_state.current_playtime += 1;
                            }
                            app_handle
                                .emit("playtime", current_playtime + 1)
                                .map_err(|_| "Error happened while emitting playtime")?;
                        }

                        thread::sleep(Duration::from_secs(1));
                    }
                    None => {
                        // Inlined close logic
                        let store = GamesStore::new(&app_handle)
                            .map_err(|_| "Error happened while accessing store")?;

                        store
                            .update_playtime(&game_id, current_playtime % 60)
                            .map_err(|_| "Error happened while setting new playtime")?;

                        store
                            .update_last_played(&game_id)
                            .map_err(|_| "Error happened while updating last played")?;

                        let state = app_handle.state::<Mutex<AppState>>();
                        let mut state = state
                            .lock()
                            .map_err(|_| "Error happened while acquiring mutex lock")?;
                        state.game = None;

                        if let Some(pres) = &mut state.presence {
                            pres.reset()
                                .map_err(|_| "Error happened while clearing presence")?;
                        }

                        app_handle.emit("current_game", json!(null)).expect("here");

                        break;
                    }
                }
            }

            Result::<(), String>::Ok(())
        });
    }
}
