use crate::{services, AppState, GameState};
use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use tauri_plugin_shell::ShellExt;
use tauri_plugin_store::StoreExt;

/// Opens a game and sets its PID in local state
#[tauri::command]
pub fn open_game(app_handle: AppHandle, game_id: String) -> Result<(), String> {
    let store = app_handle
        .store("store.json")
        .map_err(|_| "Couldn't access games data")?;

    let games = store.get("gamesData").unwrap();

    if let Some(game) = games.get(&game_id) {
        let exe_path = game
            .get("exe_file_path")
            .expect("Should be a valid exe path")
            .as_str()
            .expect("Should be a string");

        tauri::async_runtime::block_on(async move {
            let (_, process) = app_handle
                .shell()
                .command(exe_path)
                .spawn()
                .expect("Error happened while running the game");

            {
                let state = app_handle.state::<Mutex<AppState>>();
                let mut state = state
                    .lock()
                    .expect("Error happened while acquiring mutex lock");

                state.game = Some({
                    GameState {
                        pid: process.pid(),
                        id: game_id,
                        ..Default::default()
                    }
                });
            }

            services::spawn_playtime_thread(app_handle);
        });
    }

    Ok(())
}
