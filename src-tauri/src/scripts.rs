use crate::prelude::{Fetchable, Result};
use crate::services::state::{AppState, ManagedState};
use crate::services::stores::games::GamesStore;
use crate::services::vndb::{VNDB_MAX_PAGE_SIZE, Vndb};
use crate::services::{
    discord::DiscordPresence, stores::games::Game, stores::settings::SettingsStore,
};
use anyhow::{Context, bail};
use log::{debug, error, info, warn};
use std::{fs, sync::Mutex};
use tauri::{AppHandle, Manager};
use tauri_plugin_fs::FsExt;
use tauri_plugin_store::StoreExt;

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
    let store = app_handle
        .store("store.json")
        .context("Failed to access store.json")?;
    let mut binding = match store.get("gamesData") {
        Some(data) => data.clone(),
        None => {
            info!("No gamesData found in store, skipping schema setup");
            return Ok(());
        }
    };

    let games = binding
        .as_object_mut()
        .context("Failed to get gamesData as an object from store")?;

    let default_game_val =
        serde_json::to_value(Game::default()).context("Failed to serialize default Game struct")?;
    let default_game = default_game_val
        .as_object()
        .context("Failed to get default Game as an object")?;

    let mut updated_games = 0;
    let mut updated_fields = 0;

    for (game_id, game_value) in games.iter_mut() {
        debug!("Checking game schema for: {}", game_id);
        let game = game_value
            .as_object_mut()
            .context(format!("Failed to get game {} as an object", game_id))?;

        let mut game_updated = false;

        // Migrate old alt_title: "", null, or string to Fetchable enum
        if let Some(alt_title_val) = game.get_mut("alt_title") {
            if alt_title_val.is_null() {
                *alt_title_val = serde_json::to_value(Fetchable::<String>::NotFound)?;
                game_updated = true;
                updated_fields += 1;
            } else if let Some(s) = alt_title_val.as_str() {
                if s.is_empty() {
                    *alt_title_val = serde_json::to_value(Fetchable::<String>::NotFetched)?;
                } else {
                    *alt_title_val = serde_json::to_value(Fetchable::Available(s.to_string()))?;
                }
                game_updated = true;
                updated_fields += 1;
            }
        }

        // Migrate old jiten_char_count: null to Fetchable::NotFetched
        if let Some(jiten_val) = game.get_mut("jiten_char_count") {
            if jiten_val.is_null() {
                *jiten_val = serde_json::to_value(Fetchable::<u64>::NotFetched)?;
                game_updated = true;
                updated_fields += 1;
            } else if let Some(n) = jiten_val.as_u64() {
                *jiten_val = serde_json::to_value(Fetchable::Available(n))?;
                game_updated = true;
                updated_fields += 1;
            }
        }

        for (k, v) in default_game {
            if !game.contains_key(k) {
                game.insert(k.clone(), v.clone());
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

    if updated_games > 0 {
        store.set("gamesData", binding);
        store
            .save()
            .context("Failed to save updated games data to store")?;
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

    let app_local_data_dir = app_handle
        .path()
        .app_local_data_dir()
        .context("Failed to get app local data directory")?;

    let path = app_local_data_dir.join("images");
    debug!("Images folder path: {:?}", path);

    if let Err(err) = fs::create_dir_all(&path) {
        if err.kind() != std::io::ErrorKind::AlreadyExists {
            bail!("Failed to create images directory {:?}: {:?}", path, err);
        } else {
            debug!("Images directory already exists: {:?}", path);
        }
    } else {
        info!("Successfully created images directory: {:?}", path);
    }

    let scope = app_handle.fs_scope();
    scope
        .allow_directory(&path, true)
        .context("Failed to allow images directory access")?;

    debug!("Images directory access granted to filesystem scope");
    Ok(())
}

/// Initializes app state
pub fn initialize_state(app_handle: &AppHandle) -> Result<()> {
    info!("Initializing application state");

    let settings_store =
        SettingsStore::new(app_handle).context("Failed to create settings store")?;
    let settings = settings_store.load().context("Failed to load settings")?;

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

    app_handle.manage(Mutex::new(ManagedState::new(state)));
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

pub fn spawn_background_tasks(app_handle: &AppHandle) {
    info!("Spawning background task for data fetching");
    fetch_missing_game_data(app_handle);
}

/// Fetches missing alt titles and Jiten character counts for all games in the store.
pub fn fetch_missing_game_data(app_handle: &AppHandle) {
    let app_handle = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        let store = match GamesStore::new(&app_handle) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to create GamesStore for background fetch: {}", e);
                return;
            }
        };

        let all_games = match store.get_all() {
            Ok(games) => games,
            Err(e) => {
                error!("Failed to get games for background fetch: {}", e);
                return;
            }
        };

        let alt_titles_to_fetch: Vec<String> = all_games
            .iter()
            .filter(|(_, game)| game.alt_title == Fetchable::NotFetched)
            .map(|(id, _)| id.clone())
            .collect();

        let jiten_counts_to_fetch: Vec<String> = all_games
            .iter()
            .filter(|(_, game)| game.jiten_char_count == Fetchable::NotFetched)
            .map(|(id, _)| id.clone())
            .collect();

        if alt_titles_to_fetch.is_empty() && jiten_counts_to_fetch.is_empty() {
            info!("Background fetch: all games are up to date.");
            return;
        }

        // Fetch Alt Titles
        let alt_title_results = async {
            if alt_titles_to_fetch.is_empty() {
                return Vec::new();
            }
            info!(
                "Attempting to fetch missing alt titles for {} games",
                alt_titles_to_fetch.len()
            );

            let mut results = Vec::new();
            let total_ids = alt_titles_to_fetch.len();
            let fetch_iterations = total_ids.div_ceil(VNDB_MAX_PAGE_SIZE);

            for i in 0..fetch_iterations {
                let start = i * VNDB_MAX_PAGE_SIZE;
                let end = std::cmp::min(start + VNDB_MAX_PAGE_SIZE, total_ids);
                let ids_slice = &alt_titles_to_fetch[start..end];

                match Vndb::get_vns_alt_title(ids_slice).await {
                    Ok(games_chunk) => {
                        for fetched_game in games_chunk {
                            let state = match fetched_game.alttitle {
                                Some(title) if !title.is_empty() => Fetchable::Available(title),
                                _ => Fetchable::NotFound,
                            };
                            results.push((fetched_game.id, state));
                        }
                    }
                    Err(e) => {
                        warn!("Failed to fetch a chunk of alt titles: {}", e);
                    }
                }
            }
            info!("Alt title fetch completed.");
            results
        }
        .await;

        // Fetch Jiten Counts
        let jiten_results = async {
            use crate::commands::jiten::fetch_jiten_char_count;
            if jiten_counts_to_fetch.is_empty() {
                return Vec::new();
            }
            info!(
                "Attempting to fetch missing Jiten counts for {} games",
                jiten_counts_to_fetch.len()
            );

            let mut results = Vec::new();
            for game_id in &jiten_counts_to_fetch {
                match fetch_jiten_char_count(app_handle.clone(), game_id.clone()).await {
                    Ok(Some(count)) => {
                        results.push((game_id, Fetchable::Available(count)));
                    }
                    Ok(None) => {
                        results.push((game_id, Fetchable::NotFound));
                    }
                    Err(e) => {
                        warn!("Failed to fetch Jiten count for {}: {}", game_id, e);
                    }
                }
            }
            info!("Jiten fetch completed.");
            results
        }
        .await;

        // Apply all updates sequentially
        info!("Applying fetched data to the store.");
        for (id, alt_title) in alt_title_results {
            if let Err(e) = store.update_game(&id, |g| g.alt_title = alt_title) {
                error!("Failed to save alt title for {}: {}", id, e);
            }
        }

        let mut jiten_updated_count = 0;
        let jiten_to_fetch_count = jiten_counts_to_fetch.len();
        for (id, jiten_count) in jiten_results {
            if let Err(e) = store.update_game(id, |g| g.jiten_char_count = jiten_count) {
                error!("Failed to save Jiten count for {}: {}", id, e);
            } else {
                jiten_updated_count += 1;
            }
        }

        if jiten_to_fetch_count > 0 {
            if jiten_updated_count > 0 {
                info!(
                    "Jiten fetch completed: updated {} games",
                    jiten_updated_count
                );
            } else {
                info!("Jiten fetch completed: No new counts found.");
            }
        }

        info!("Background data fetch task completed.");
    });
}
