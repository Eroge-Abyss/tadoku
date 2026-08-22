use crate::prelude::{Fetchable, Result, Store};
use crate::services::stores::games::Game;
use anyhow::Context;
use log::{debug, info, warn};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreExt;

const CURRENT_VERSION: u32 = 1;
const VERSION_KEY: &str = "schemaVersion";
const STORE_FILENAME: &str = "store.json";

pub fn migrate(app_handle: &AppHandle) -> Result<()> {
    // tauri-plugin-store silently swallows deserialization errors when it
    // opens a store (see StoreBuilder::build_inner, `let _ = store_inner.load();`).
    // If store.json exists but is not valid JSON, the plugin hands us an
    // *empty* in-memory store instead of failing. Our migration code would
    // then treat that as "no data yet", run happily, and overwrite the
    // corrupt file with a near-empty one on the very next save — destroying
    // whatever was recoverable in the original file.
    //
    // So: check the file ourselves, first, before the plugin ever touches it.
    guard_against_corrupt_store(app_handle)?;

    let store = app_handle
        .store(STORE_FILENAME)
        .context("Failed to access store.json")?;

    let version = read_version(&store);

    if version < CURRENT_VERSION {
        run_migrations(&store, version)?;
        write_version(&store, CURRENT_VERSION)?;
    }

    Ok(())
}

/// If store.json exists on disk but fails to parse as JSON, rename it out of
/// the way and bail out with an error instead of letting the app silently
/// treat it as empty. This preserves the corrupt file for manual recovery
/// and surfaces the problem instead of hiding it.
fn guard_against_corrupt_store(app_handle: &AppHandle) -> Result<()> {
    let path = app_handle
        .path()
        .resolve(STORE_FILENAME, BaseDirectory::AppData)
        .context("Failed to resolve store.json path")?;

    if !path.exists() {
        // Genuinely new install / first run. Nothing to guard.
        return Ok(());
    }

    let bytes = fs::read(&path).context("Failed to read store.json for validation")?;

    if serde_json::from_slice::<serde_json::Value>(&bytes).is_err() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let backup_path = path.with_file_name(format!("store.json.corrupt-{timestamp}"));

        fs::rename(&path, &backup_path)
            .context("store.json is corrupt and could not be moved aside for recovery")?;

        warn!(
            "store.json failed to parse as JSON. Moved it to {} to prevent data loss. \
             The app will start with a fresh store; the original file was preserved for recovery.",
            backup_path.display()
        );

        // We do NOT delete or silently continue past this, we surface it,
        // and a fresh, empty store.json will be created on next save, but
        // the *original* file survives on disk under the .corrupt- name
        // rather than being overwritten.
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
