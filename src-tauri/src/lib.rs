use services::discord::DiscordPresence;
use std::{fs, sync::Mutex};
use tauri::Manager;
use tauri_plugin_fs::FsExt;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod services;
mod util;

#[derive(Default, Clone)]
struct GameState {
    id: String,
    pid: u32,
    current_playtime: u64,
}

#[derive(Default)]
struct AppState {
    game: Option<GameState>,
    presence: Option<DiscordPresence>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Create state
            let state = AppState {
                presence: DiscordPresence::new().ok(),
                ..Default::default()
            };

            app.manage(Mutex::new(state));

            // Create images folder if it doesn't exist
            if let Ok(app_local_data_dir) = app.path().app_local_data_dir() {
                let path = app_local_data_dir.join("images");

                if let Err(err) = fs::create_dir_all(&path) {
                    if err.kind() != std::io::ErrorKind::AlreadyExists {
                        return Err(Box::new(err));
                    }
                }

                let scope = app.fs_scope();
                scope
                    .allow_directory(path, true)
                    .expect("Should allow images directory to be accessed");
            } else {
                return Err("Failed to get app local data directory".into());
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::vndb::fetch_vn_info,
            commands::storage::save_game,
            commands::storage::load_games,
            commands::storage::delete_game,
            commands::storage::toggle_pin,
            commands::opener::open_game
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
