use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod scripts;
mod services;
mod util;

pub use scripts::{AppState, GameState};

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
            scripts::setup_store(&app.app_handle())?;
            scripts::create_images_folder(&app.app_handle())?;
            scripts::initialize_state(&app.app_handle());

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
