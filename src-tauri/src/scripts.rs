use crate::prelude::Result;
use crate::services::vndb::{Vndb, VndbAltTitleGame, VNDB_MAX_PAGE_SIZE};
use crate::services::{
    discord::DiscordPresence,
    stores::games::Game,
    stores::settings::{Settings, SettingsStore},
};
use log::{debug, error, info, warn};
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
pub struct AppState {
    pub game: Option<GameState>,
    pub presence: Option<DiscordPresence>,
    pub settings: Settings,
}

impl AppState {
    pub fn update_settings<F>(&mut self, app_handle: &AppHandle, update_fn: F) -> Result<()>
    where
        F: FnOnce(&mut Settings),
    {
        update_fn(&mut self.settings);

        let store = SettingsStore::new(app_handle)?;
        store.save(&self.settings)?;

        Ok(())
    }
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
    info!("Setting up store schema compatibility");
    let store = app_handle.store("store.json").map_err(|e| {
        error!("Failed to access store.json: {:?}", e);
        e
    })?;
    let mut binding = store.get("gamesData");

    if binding.is_none() {
        info!("No gamesData found in store, skipping schema setup");
        return Ok(());
    }

    let games = binding
        .as_mut()
        .and_then(|v| v.as_object_mut())
        .ok_or_else(|| {
            error!("Failed to get gamesData as an object from store");
            "Failed to get gamesData as an object"
        })?;

    let binding = serde_json::to_value(Game::default()).map_err(|e| {
        error!("Failed to serialize default Game struct: {:?}", e);
        e
    })?;
    let default_game = binding.as_object().ok_or_else(|| {
        error!("Failed to get default Game as an object");
        "Failed to get default Game as an object"
    })?;

    let mut updated_games = 0;
    let mut updated_fields = 0;
    let mut missing_alt_title_ids: Vec<String> = vec![];

    for (game_id, game) in games.iter_mut() {
        debug!("Checking game schema for: {}", game_id);
        let game = game.as_object_mut().ok_or_else(|| {
            error!("Failed to get game {} as an object", game_id);
            "Failed to get game as an object"
        })?;

        let mut game_updated = false;

        let alt_title = game.get("alt_title");
        if alt_title.is_none()
            || (alt_title.is_some() && alt_title.expect("Should not happen, already checked") == "")
        {
            missing_alt_title_ids.push(game_id.clone());
            game_updated = true;
        }

        for (k, v) in default_game {
            if game.get(k).is_none() {
                debug!("Adding missing field '{}' to game {}", k, game_id);
                if k == "alt_title" {
                    game.insert(k.clone(), "".into());
                } else {
                    game.insert(k.clone(), v.clone());
                }

                game_updated = true;
                updated_fields += 1;
            }
        }

        if game_updated {
            updated_games += 1;
        }

        // Handle empty process path case
        if let Some(process_path) = game.get("process_file_path") {
            if process_path == "" {
                debug!("Fixing empty process_file_path for game {}", game_id);
                if let Some(exe_path) = game.get("exe_file_path") {
                    game.insert("process_file_path".into(), exe_path.clone());
                    updated_fields += 1;
                } else {
                    warn!(
                        "Game {} has empty process_file_path but no exe_file_path",
                        game_id
                    );
                }
            }
        }
    }

    if !missing_alt_title_ids.is_empty() {
        tauri::async_runtime::block_on(async {
            let _ = add_missing_alt_titles(games, &missing_alt_title_ids).await;
        });
    }

    let games = serde_json::to_value(games).map_err(|e| {
        error!("Failed to serialize updated games data: {:?}", e);
        e
    })?;

    store.set("gamesData", games);
    store.save().map_err(|e| {
        error!("Failed to save updated games data to store: {:?}", e);
        format!("Failed to save store: {:?}", e)
    })?;

    if updated_games > 0 {
        info!(
            "Store schema setup completed: updated {} games with {} fields",
            updated_games, updated_fields
        );
    } else {
        info!("Store schema setup completed: all games up to date");
    }

    Ok(())
}

/// Creates images folder if it doesn't exist
pub fn create_images_folder(app_handle: &AppHandle) -> Result<()> {
    info!("Creating images folder");

    if let Ok(app_local_data_dir) = app_handle.path().app_local_data_dir() {
        let path = app_local_data_dir.join("images");
        debug!("Images folder path: {:?}", path);

        if let Err(err) = fs::create_dir_all(&path) {
            if err.kind() != std::io::ErrorKind::AlreadyExists {
                error!("Failed to create images directory {:?}: {:?}", path, err);
                return Err(Box::new(err));
            } else {
                debug!("Images directory already exists: {:?}", path);
            }
        } else {
            info!("Successfully created images directory: {:?}", path);
        }

        let scope = app_handle.fs_scope();
        scope.allow_directory(&path, true).map_err(|e| {
            error!("Failed to allow images directory access: {:?}", e);
            format!("Failed to allow directory access: {:?}", e)
        })?;

        debug!("Images directory access granted to filesystem scope");
        Ok(())
    } else {
        error!("Failed to get app local data directory");
        Err("Failed to get app local data directory".into())
    }
}

/// Initializes app state
pub fn initialize_state(app_handle: &AppHandle) -> Result<()> {
    info!("Initializing application state");

    let settings_store = SettingsStore::new(app_handle).map_err(|e| {
        error!("Failed to create settings store: {:?}", e);
        e
    })?;

    let settings = settings_store.load().map_err(|e| {
        error!("Failed to load settings: {:?}", e);
        e
    })?;

    debug!(
        "App config - disable_presence_on_nsfw: {}",
        settings.disable_presence_on_nsfw
    );
    debug!("App config - playtime_mode loaded successfully");

    debug!(
        "App config - use_jp_for_title_time: {}",
        settings.use_jp_for_title_time
    );

    let state = AppState {
        presence: None,
        settings,
        ..Default::default()
    };

    app_handle.manage(Mutex::new(state));
    info!("Application state initialized successfully");

    Ok(())
}

pub fn initialize_discord(app_handle: &AppHandle) -> tauri::Result<()> {
    info!("Initializing Discord presence");
    let app_handle_clone = app_handle.clone();

    tauri::async_runtime::spawn(async move {
        debug!("Starting Discord initialization background task");
        let app_state_mutex = app_handle_clone.state::<Mutex<AppState>>();

        let mut state = match app_state_mutex.lock() {
            Ok(s) => {
                debug!("Background task: Successfully acquired app state mutex lock");
                s
            }
            Err(e) => {
                error!(
                    "Background task: Error acquiring mutex lock for AppState: {}",
                    e
                );
                return Ok(());
            }
        };

        let mode = state.settings.discord_presence_mode;

        debug!("Background task: Retrieved Discord presence mode from settings");

        match DiscordPresence::new(mode) {
            Ok(presence) => {
                info!("Background task: Successfully initialized Discord presence");
                state.presence = Some(presence);
            }
            Err(e) => {
                warn!(
                    "Background task: Failed to initialize DiscordPresence: {}",
                    e
                );
                debug!("Background task: Discord presence will be disabled");
                state.presence = None;
            }
        }

        debug!("Background task: Discord initialization completed");
        std::result::Result::<(), String>::Ok(())
    });

    info!("Discord initialization task spawned");
    Ok(())
}

/// Adds missing alt title for provided VN IDs
pub async fn add_missing_alt_titles(
    games: &mut serde_json::Map<String, serde_json::Value>,
    ids: &[String],
) -> Result<()> {
    info!(
        "Attempting to add missing alt titles for {} games",
        ids.len()
    );
    let mut games_with_title: Vec<VndbAltTitleGame> = vec![];

    let total_ids = ids.len();
    let fetch_iterations = total_ids.div_ceil(VNDB_MAX_PAGE_SIZE);

    for i in 0..fetch_iterations {
        let start = i * VNDB_MAX_PAGE_SIZE;
        let end = std::cmp::min(start + VNDB_MAX_PAGE_SIZE, total_ids);
        let current_ids_slice = &ids[start..end];
        let games_chunk = Vndb::get_vns_alt_title(current_ids_slice).await?;
        games_with_title.extend(games_chunk);
    }

    for game_title in games_with_title {
        let game = games.get_mut(&game_title.id).expect("Should be there");
        debug!("Updating alt_title for game ID: {}", game_title.id);
        game["alt_title"] = game_title.alttitle.into();
    }

    info!("Successfully added missing alt titles.");
    Ok(())
}
