use crate::prelude::{Fetchable, Result, Store};
use crate::services::stores::games::Game;
use anyhow::Context;
use log::{debug, info, warn};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const CURRENT_VERSION: u32 = 1;
const VERSION_KEY: &str = "schemaVersion";

pub fn migrate(app_handle: &AppHandle) -> Result<()> {
    let store = app_handle
        .store("store.json")
        .context("Failed to access store.json")?;

    let version = read_version(&store);

    if version < CURRENT_VERSION {
        run_migrations(&store, version)?;
        write_version(&store, CURRENT_VERSION)?;
    }

    Ok(())
}

fn read_version(store: &Store) -> u32 {
    store
        .get(VERSION_KEY)
        .and_then(|v| v.as_u64())
        .map(|v| v as u32)
        .unwrap_or(0) // no key = version 0 = existing users
}

fn write_version(store: &Store, version: u32) -> Result<()> {
    store.set(VERSION_KEY, version);
    store.save().context("Failed to save schema version")
}

fn run_migrations(store: &Store, from: u32) -> Result<()> {
    if from < 1 {
        v0_to_v1(store)?;
    }
    // if from < 2 { v1_to_v2(app_handle, store)?; }
    Ok(())
}

/// Your existing migration logic, verbatim
fn v0_to_v1(store: &Store) -> Result<()> {
    info!("Running migration v0 -> v1");

    let mut binding = match store.get("gamesData") {
        Some(data) => data.clone(),
        None => {
            info!("No gamesData found, skipping v0->v1 migration");
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

    info!("Migration v0 -> v1 complete");
    Ok(())
}
