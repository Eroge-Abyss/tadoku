use crate::prelude::Result;
use crate::services::discord::DiscordPresenceMode;
use crate::services::{discord::DiscordPresence, games_store::Game, settings_store::SettingsStore};
use std::{fs, sync::Mutex};
use tauri::{AppHandle, Manager};
use tauri_plugin_fs::FsExt;
use tauri_plugin_store::StoreExt;

#[derive(Default, Clone)]
pub struct GameState {
    pub id: String,
    pub pid: u32,
    pub current_playtime: u64,
}

#[derive(Default)]
pub struct Config {
    pub disable_presence_on_nsfw: bool,
}

#[derive(Default)]
pub struct AppState {
    pub game: Option<GameState>,
    pub presence: Option<DiscordPresence>,
    pub config: Config,
}

// This script should be able to
// 1. Get the store
// 2. Loop over each item in it
// 3. Check if all keys match with the current Game struct in code
// 4. If something does not match (aka an old version), update the game with the default value which will be here
// 5. Save the updated result into the JSON store
// 6. Don't forget this should run when initializing the app

/// Makes store in sync with latest Game struct schema
pub fn setup_store(app_handle: &AppHandle) -> Result<()> {
    let store = app_handle.store("store.json")?;
    let mut binding = store.get("gamesData");

    if binding.is_none() {
        return Ok(());
    }

    let games = binding
        .as_mut()
        .and_then(|v| v.as_object_mut())
        .ok_or("Failed to get gamesData as an object")?;

    let binding = serde_json::to_value(Game::default())?;
    let default_game = binding
        .as_object()
        .ok_or("Failed to get default Game as an object")?;

    for (_, game) in games.iter_mut() {
        let game = game
            .as_object_mut()
            .ok_or("Failed to get game as an object")?;

        for (k, v) in default_game {
            if game.get(k).is_none() {
                game.insert(k.clone(), v.clone());
            }
        }

        // Handle empty process path case
        if game.get("process_file_path").expect("Should have this key") == "" {
            game.insert(
                "process_file_path".into(),
                game.get("exe_file_path")
                    .expect("Should have this key")
                    .clone(),
            );
        }
    }

    let games = serde_json::to_value(games)?;
    store.set("gamesData", games);

    Ok(())
}

/// Creates images folder if it doesn't exist
pub fn create_images_folder(app_handle: &AppHandle) -> Result<()> {
    if let Ok(app_local_data_dir) = app_handle.path().app_local_data_dir() {
        let path = app_local_data_dir.join("images");

        if let Err(err) = fs::create_dir_all(&path) {
            if err.kind() != std::io::ErrorKind::AlreadyExists {
                return Err(Box::new(err));
            }
        }

        let scope = app_handle.fs_scope();
        scope
            .allow_directory(path, true)
            .expect("Should allow images directory to be accessed");

        Ok(())
    } else {
        Err("Failed to get app local data directory".into())
    }
}

/// Initializes app state
pub fn initialize_state(app_handle: &AppHandle) -> Result<()> {
    let settings_store = SettingsStore::new(app_handle)?;
    let config = Config {
        disable_presence_on_nsfw: settings_store.get_presence_on_nsfw()?,
    };

    let state = AppState {
        presence: None,
        config,
        ..Default::default()
    };

    app_handle.manage(Mutex::new(state));

    Ok(())
}

pub fn initialize_discord(app_handle: &AppHandle) -> tauri::Result<()> {
    let app_handle_clone = app_handle.clone();

    tauri::async_runtime::spawn(async move {
        let app_state_mutex = app_handle_clone.state::<Mutex<AppState>>();

        let mut state = match app_state_mutex.lock() {
            Ok(s) => s,
            Err(e) => {
                eprintln!(
                    "Background task: Error acquiring mutex lock for AppState: {}",
                    e
                );
                return;
            }
        };

        match DiscordPresence::new(DiscordPresenceMode::default()) {
            Ok(presence) => {
                state.presence = Some(presence);
            }
            Err(e) => {
                eprintln!(
                    "Background task: Failed to initialize DiscordPresence: {}",
                    e
                );
                state.presence = None;
            }
        }
    });

    Ok(())
}
