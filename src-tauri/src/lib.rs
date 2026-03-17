use anyhow::Context;
use log::{error, info};
use tauri::{AppHandle, Manager, RunEvent};
use tokio_util::sync::CancellationToken;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
mod prelude;
mod scripts;
mod services;
mod util;

pub use scripts::{AppState, GameState};
use services::playtime;

struct ShutdownToken(CancellationToken);

fn setup_app(app: &AppHandle) -> anyhow::Result<()> {
    scripts::setup_store(app).context("Failed to setup store")?;
    scripts::create_images_folder(app).context("Failed to create images folder")?;
    scripts::initialize_state(app).context("Failed to initialize state")?;
    scripts::initialize_discord(app).context("Failed to initialize Discord")?;

    let token = app.state::<ShutdownToken>().0.clone();
    playtime::ExStaticPlaytime::spawn(app, token);
    scripts::spawn_background_tasks(app);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let default_log_level = if util::is_debug_mode() {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .filter(|metadata| metadata.target().starts_with("tadoku_lib"))
                .level(default_log_level)
                .max_file_size(100_000) // ~100KB
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
        .manage(ShutdownToken(CancellationToken::new()))
        .setup(|app| {
            info!("Starting Tadoku application");
            if let Err(e) = setup_app(app.handle()) {
                error!("Failed to setup application: {:?}", e);
                // The returned error will be presented to the user by Tauri.
                return Err(e.into());
            }
            info!("Tadoku application setup completed successfully");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::vndb::fetch_vn_info,
            commands::storage::save_game,
            commands::storage::load_games,
            commands::storage::delete_game,
            commands::storage::toggle_pin,
            commands::storage::reset_stats,
            commands::storage::update_exe,
            commands::storage::set_game_categories,
            commands::storage::update_process,
            commands::storage::get_categories,
            commands::storage::set_categories,
            commands::storage::get_selected_categories,
            commands::storage::set_selected_categories,
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
            commands::storage::get_use_jp_for_title_time,
            commands::storage::set_use_jp_for_title_time,
            commands::storage::get_hide_nsfw_images,
            commands::storage::set_hide_nsfw_images,
            commands::storage::get_jiten_base_url,
            commands::storage::set_jiten_base_url,
            commands::opener::open_game,
            commands::opener::close_game,
            commands::opener::get_active_windows
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|app_handle, event| {
        if let RunEvent::ExitRequested { .. } = event {
            info!("Exit requested, signaling shutdown.");
            let token = app_handle.state::<ShutdownToken>().0.clone();
            token.cancel();
            // Give tasks a moment to shut down before the app forcefully closes.
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    });
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
