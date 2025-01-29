use crate::util::get_playtime;
use crate::AppState;
use serde_json::json;
use std::{sync::Mutex, thread, time::Duration};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_store::StoreExt;

pub fn spawn_playtime_thread(app_handle: AppHandle) {
    // TODO: Add better error handling with -> Result
    tauri::async_runtime::spawn(async move {
        loop {
            let (pid, game_id, current_playtime) = {
                let state = app_handle.state::<Mutex<AppState>>();
                let mut state = state
                    .lock()
                    .expect("Error happened while acquiring mutex lock");
                let game_state = state.game.as_mut().expect("Couldn't find the game");

                (
                    game_state.pid,
                    game_state.id.clone(),
                    game_state.current_playtime,
                )
            };

            match get_playtime(pid) {
                Some(time) => {
                    {
                        let state = app_handle.state::<Mutex<AppState>>();
                        let mut state = state
                            .lock()
                            .expect("Error happened while acquiring mutex lock");
                        let game_state = state.game.as_mut().expect("Couldn't find the game");
                        game_state.current_playtime = time;
                    }

                    app_handle
                        .emit("playtime", time)
                        .expect("Error happened while emitting playtime");

                    thread::sleep(Duration::from_secs(1));
                }
                None => {
                    let store = app_handle.store("store.json").expect("Couldn't open store");
                    let mut games_data = store.get("gamesData").unwrap();
                    let game = games_data.get_mut(&game_id).unwrap();

                    game["playtime"] = json!(game["playtime"].as_u64().unwrap() + current_playtime);
                    store.set("gamesData", games_data);

                    let state = app_handle.state::<Mutex<AppState>>();
                    let mut state = state
                        .lock()
                        .expect("Error happened while acquiring mutex lock");
                    state.game = None;

                    if let Some(pres) = &mut state.presence {
                        pres.clear()
                            .expect("Error happened while clearing presence");
                    }

                    break;
                }
            }
        }
    });
}
