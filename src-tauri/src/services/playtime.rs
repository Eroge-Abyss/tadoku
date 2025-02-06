use super::store::GamesStore;
use crate::util::get_playtime;
use crate::AppState;
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

            match get_playtime(pid) {
                Some(time) => {
                    const FLUSH_DURATION: u8 = 60;

                    thread::sleep(Duration::from_secs(FLUSH_DURATION.into()));

                    {
                        let state = app_handle.state::<Mutex<AppState>>();
                        let mut state = state
                            .lock()
                            .map_err(|_| "Error happened while acquiring mutex lock")?;
                        let game_state = state.game.as_mut().ok_or("Couldn't find the game")?;
                        game_state.current_playtime = time;
                    }

                    let store = GamesStore::new(&app_handle)
                        .map_err(|_| "Error happened while accessing store")?;

                    store
                        .update_playtime(&game_id, FLUSH_DURATION.into())
                        .map_err(|_| "Error happened while setting new playtime")?;

                    app_handle
                        .emit("playtime", time)
                        .map_err(|_| "Error happened while emitting playtime")?;
                }
                None => {
                    let store = GamesStore::new(&app_handle)
                        .map_err(|_| "Error happened while accessing store")?;

                    store
                        .update_playtime(&game_id, current_playtime % 60)
                        .map_err(|_| "Error happened while setting new playtime")?;

                    let state = app_handle.state::<Mutex<AppState>>();
                    let mut state = state
                        .lock()
                        .map_err(|_| "Error happened while acquiring mutex lock")?;
                    state.game = None;

                    if let Some(pres) = &mut state.presence {
                        pres.clear()
                            .map_err(|_| "Error happened while clearing presence")?;
                    }

                    break;
                }
            }
        }

        Result::<(), String>::Ok(())
    });
}
