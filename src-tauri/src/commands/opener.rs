use crate::{
    services::{discord::DiscordGameDetails, playtime, store::GamesStore},
    AppState, GameState,
};
use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use tauri_plugin_shell::ShellExt;

/// Opens a game and sets its PID in local state
#[tauri::command]
pub fn open_game(app_handle: AppHandle, game_id: String) -> Result<(), String> {
    let store = GamesStore::new(&app_handle).map_err(|_| "Error happened while accessing store")?;

    if let Some(game) = store.get(&game_id) {
        let exe_path = game.exe_file_path;

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
                        id: game_id.clone(),
                        ..Default::default()
                    }
                });

                if let Some(pres) = &mut state.presence {
                    pres.set(DiscordGameDetails::new(game_id, game.title, game.image_url))
                        .expect("Error happened while setting presence");
                }
            }

            playtime::spawn_playtime_thread(app_handle);
        });
    }

    Ok(())
}
