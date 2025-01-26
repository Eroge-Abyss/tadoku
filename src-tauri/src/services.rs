use crate::util::get_playtime;
use crate::AppState;
use serde_json::json;
use std::{sync::Mutex, thread::sleep, time::Duration};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_store::StoreExt;

/// Spawns a thread for playtime stats
pub fn spawn_playtime_thread(app_handle: AppHandle) {
    // TODO: Add better error handling with -> Result
    tauri::async_runtime::spawn(async move {
        let state = app_handle.state::<Mutex<AppState>>();
        let mut state = state
            .lock()
            .expect("Error happened while acquiring mutex lock");
        let game_state = state.game.as_mut().expect("Couldn't find the game");

        loop {
            match get_playtime(game_state.pid) {
                Some(time) => {
                    game_state.current_playtime = time;
                    app_handle
                        .emit("playtime", time)
                        .expect("Error happened while emitting playtime");

                    sleep(Duration::from_secs(1));
                }
                None => {
                    let store = app_handle.store("store.json").expect("Couldn't open store");
                    let mut games_data = store.get("gamesData").unwrap();
                    let game = games_data.get_mut(&game_state.id).unwrap();

                    game["play_time"] =
                        json!(game["play_time"].as_u64().unwrap() + game_state.current_playtime);
                    store.set("gamesData", games_data);

                    state.game = None;
                    break;
                }
            }
        }
    });
}
