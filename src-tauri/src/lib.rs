use log::{error, info};
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod prelude;
mod scripts;
mod services;
mod util;

pub use scripts::{AppState, GameState};
use services::playtime;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(prevent_default())
        .setup(|app| {
            info!("Starting Tadoku application");

            if let Err(e) = scripts::setup_store(app.app_handle()) {
                error!("Failed to setup store: {}", e);
                return Err(e);
            }

            if let Err(e) = scripts::create_images_folder(app.app_handle()) {
                error!("Failed to create images folder: {}", e);
                return Err(e);
            }

            if let Err(e) = scripts::initialize_state(app.app_handle()) {
                error!("Failed to initialize state: {}", e);
                return Err(e);
            }

            if let Err(e) = scripts::initialize_discord(app.app_handle()) {
                error!("Failed to initialize Discord: {}", e);
                return Err(Box::new(e));
            }

            playtime::ExStaticPlaytime::spawn(app.app_handle());
            info!("Tadoku application setup completed successfully");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::vndb::fetch_vn_info,
            commands::storage::save_game,
            commands::storage::load_games,
            commands::storage::delete_game,
            commands::storage::toggle_pin,
            commands::storage::update_exe,
            commands::storage::set_game_categories,
            commands::storage::update_process,
            commands::storage::get_categories,
            commands::storage::set_categories,
            commands::storage::get_theme_settings,
            commands::storage::set_theme_settings,
            commands::storage::get_nsfw_presence_status,
            commands::storage::set_nsfw_presence_status,
            commands::storage::get_show_random_picker,
            commands::storage::set_show_random_picker,
            commands::storage::get_discord_presence_mode,
            commands::storage::set_discord_presence_mode,
            commands::storage::get_playtime_mode,
            commands::storage::set_playtime_mode,
            commands::storage::get_sort_order,
            commands::storage::set_sort_order,
            commands::storage::set_characters,
            commands::storage::set_game_notes,
            commands::opener::open_game,
            commands::opener::close_game,
            commands::opener::get_active_windows
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(debug_assertions)]
fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    use tauri_plugin_prevent_default::Flags;

    tauri_plugin_prevent_default::Builder::new()
        .with_flags(Flags::all().difference(Flags::CONTEXT_MENU))
        .build()
}

#[cfg(not(debug_assertions))]
fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    tauri_plugin_prevent_default::init()
}
