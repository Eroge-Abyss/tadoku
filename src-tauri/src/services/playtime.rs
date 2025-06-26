use super::stores::games::GamesStore;
#[cfg(windows)]
use crate::util::flush_playtime;
use crate::util::get_playtime;
use crate::AppState;
use serde_json::json;
use std::{sync::Mutex, thread, time::Duration};
use tauri::{AppHandle, Emitter, Manager};

pub fn spawn_playtime_thread(app_handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        loop {
            let (pid, game_id, current_playtime) = {
                let state = app_handle.state::<Mutex<AppState>>();
                let mut state = state
                    .lock()
                    .map_err(|_| "Error happened while acquiring mutex lock")?;
                let game_state = state.game.as_mut().ok_or("Couldn't find the game")?;

                (
                    game_state.pid,
                    game_state.id.clone(),
                    game_state.current_playtime,
                )
            };

            // TODO: Do some DRY here

            match get_playtime(pid) {
                Some(_) => {
                    #[cfg(windows)]
                    {
                        if let Ok(active_window) = x_win::get_active_window() {
                            if active_window.id == pid {
                                // Should exec only if current window is active window
                                {
                                    let state = app_handle.state::<Mutex<AppState>>();
                                    let mut state = state
                                        .lock()
                                        .map_err(|_| "Error happened while acquiring mutex lock")?;
                                    let game_state =
                                        state.game.as_mut().ok_or("Couldn't find the game")?;
                                    game_state.current_playtime += 1;
                                }

                                if current_playtime % 60 == 0 {
                                    flush_playtime(&app_handle, &game_id, 60)
                                        .map_err(|_| "Error happened while updating playtime")?;
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
                            let game_state = state.game.as_mut().ok_or("Couldn't find the game")?;
                            game_state.current_playtime += 1;
                        }
                        app_handle
                            .emit("playtime", current_playtime + 1)
                            .map_err(|_| "Error happened while emitting playtime")?;
                    }

                    thread::sleep(Duration::from_secs(1));
                }
                None => {
                    let store = GamesStore::new(&app_handle)
                        .map_err(|_| "Error happened while accessing store")?;

                    store
                        .update_playtime(&game_id, current_playtime % 60)
                        .map_err(|_| "Error happened while setting new playtime")?;

                    store
                        .update_last_played(&game_id)
                        .map_err(|_| "Error happened while updating last played")?;

                    store
                        .set_first_played(&game_id)
                        .map_err(|_| "Error happened while setting first played")?;

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
