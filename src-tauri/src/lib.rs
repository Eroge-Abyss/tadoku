use std::{fs, sync::Mutex};
use tauri::Manager;
use tauri_plugin_fs::FsExt;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod playtime;
mod storage;
mod util;
mod vndb;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Default)]
struct AppState {
    game_pid: u32,
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
            // Create game pid state
            app.manage(Mutex::new(AppState::default()));

            // Create images folder if it doesn't exist
            if let Ok(app_local_data_dir) = app.path().app_local_data_dir() {
                let path = app_local_data_dir.join("images");

                if let Err(err) = fs::create_dir_all(&path) {
                    if err.kind() != std::io::ErrorKind::AlreadyExists {
                        return Err(Box::new(err));
                    }
                }

                let scope = app.fs_scope();
                scope.allow_directory(path, true).unwrap();
            } else {
                return Err("Failed to get app local data directory".into());
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            vndb::fetch_vn_info,
            storage::save_game,
            storage::load_games,
            storage::delete_game,
            playtime::open_game
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
