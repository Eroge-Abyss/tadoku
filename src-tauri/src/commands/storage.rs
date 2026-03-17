use crate::commands::cmd_result::CmdResult;
use crate::commands::jiten;
use crate::prelude::Fetchable;
use crate::{
    AppState,
    services::{
        discord::DiscordPresenceMode,
        stores::{
            categories::{Categories, CategoriesStore},
            games::{Character, Game, Games, GamesStore},
            settings::{PlaytimeMode, SortOrder, ThemeSettings},
        },
        vndb::Vndb,
    },
    util::{self},
};
use anyhow::Context;
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use tokio;
#[cfg(windows)]
use windows_icons;

#[derive(Debug, Serialize, Deserialize)]
pub struct Options {
    include_characters: bool,
}

/// Saves a game to the local storage.
///
/// **NOTE**: The image is either downloaded from a remote URL or copied from a
/// local path.  When `image_url` is empty (manual entry with no cover) the
/// image step is skipped entirely.
#[tauri::command]
pub async fn save_game(
    app_handle: AppHandle,
    game_id: String,
    mut game: Game,
    options: Options,
) -> CmdResult<()> {
    info!("Saving game: {} ({})", game.title, game_id);
    debug!(
        "Game save options - include_characters: {}",
        options.include_characters
    );

    // Save image (skip when no image was provided).
    let _path: Option<String> = if game.image_url.is_empty() {
        debug!(
            "No image URL provided for game {}, skipping image save",
            game_id
        );
        None
    } else {
        // For local paths generate a unique dest name; for remote URLs derive from the URL.
        let dest_name = util::is_local_path(&game.image_url).then(|| {
            let filename =
                util::extract_image(&game.image_url).unwrap_or_else(|_| "cover.jpg".to_owned());
            format!("{}_{}", game_id, filename)
        });
        let source = game.image_url.clone();
        // Update stored image_url to the final filename before saving.
        if let Some(ref name) = dest_name {
            game.image_url = name.clone();
        }
        let path = util::save_image(&app_handle, &source, dest_name.as_deref())
            .await
            .context("Error happened while saving image")?;
        debug!("Successfully saved game image for {}", game_id);
        Some(path)
    };

    #[cfg(windows)]
    if let Some(ref saved_path) = _path {
        debug!("Extracting and saving icon for game {}", game_id);
        let icon = windows_icons::get_icon_by_path(&game.exe_file_path);
        let icon_path = format!("{}.icon.png", saved_path);
        icon.save(&icon_path)
            .context("Error happened while saving image")?;

        game.icon_url = Some(icon_path);
        debug!("Successfully saved icon for game {}", game_id);
    }

    #[cfg(not(windows))]
    {
        debug!(
            "Not on Windows, skipping icon extraction for game {}",
            game_id
        );
        game.icon_url = None;
    }

    let jiten_char_count_fut = jiten::fetch_jiten_char_count(app_handle.clone(), game_id.clone());

    let characters_fut = async {
        if !options.include_characters {
            debug!("Skipping character fetching for game {}", game_id);
            return Ok::<_, anyhow::Error>(None);
        }

        info!("Fetching characters for game {}", game_id);
        let chars = Vndb::get_vn_characters(&game_id)
            .await
            .context(format!("Error fetching characters for game {}", game_id))?;
        debug!("Found {} characters for game {}", chars.len(), game_id);

        let mut new_chars: Vec<Character> = Vec::new();

        for char in chars {
            debug!("Processing character: {} (ID: {})", char.name, char.id);
            let path = match char.image {
                Some(p) => {
                    debug!("Saving character image for {} ({})", char.name, p.url);
                    Some(
                        util::save_image(&app_handle, &p.url, None)
                            .await
                            .context("Error happened while saving image")?,
                    )
                }
                None => {
                    debug!("No image found for character: {}", char.name);
                    None
                }
            };

            new_chars.push(Character {
                id: char.id,
                en_name: char.name,
                og_name: char.original,
                image_url: path,
            });
        }

        info!(
            "Successfully processed {} characters for game {}",
            new_chars.len(),
            game_id
        );
        Ok(Some(new_chars))
    };

    let (jiten_result, characters_result) = tokio::join!(jiten_char_count_fut, characters_fut);

    game.jiten_char_count = match jiten_result {
        Ok(Some(count)) => {
            info!(
                "Successfully fetched Jiten character count ({}) for game {}",
                count, game_id
            );
            Fetchable::Available(count)
        }
        Ok(None) => {
            info!("No Jiten character count found for game {}", game_id);
            Fetchable::NotFound
        }
        Err(e) => {
            warn!("Jiten fetch failed for {}: {}", game_id, e);
            Fetchable::NotFetched
        }
    };

    game.characters = characters_result?;

    let store = GamesStore::new(&app_handle).context("Error happened while accessing store")?;

    store
        .save(game_id.clone(), game)
        .context("Error happened while saving game")?;

    info!("Successfully saved game: {}", game_id);
    Ok(())
}

/// Loads all games from JSON storage
#[tauri::command]
pub fn load_games(app_handle: AppHandle) -> CmdResult<Games> {
    debug!("Loading all games from storage");
    let store = GamesStore::new(&app_handle).context("Error happened while accessing store")?;

    let games_data = store
        .get_all()
        .context("Error happened while getting games")?;

    debug!(
        "Successfully loaded {} games from storage",
        games_data.len()
    );
    Ok(games_data)
}

/// Deletes a game from JSON storage
#[tauri::command]
pub fn delete_game(app_handle: AppHandle, game_id: String) -> CmdResult<()> {
    info!("Deleting game: {}", game_id);
    let store = GamesStore::new(&app_handle).context("Error happened while accessing store")?;

    store
        .delete(&game_id)
        .context("Error happened while deleting game")?;

    info!("Successfully deleted game: {}", game_id);
    Ok(())
}

/// Toggles the pinned state of a game
#[tauri::command]
pub fn toggle_pin(app_handle: AppHandle, game_id: String) -> CmdResult<()> {
    debug!("Toggling pin state for game: {}", game_id);
    let store = GamesStore::new(&app_handle).context("Error happened while accessing store")?;

    store
        .update_game(&game_id, |g| g.is_pinned = !g.is_pinned)
        .context("Error happened while toggling pin")?;

    info!("Successfully toggled pin state for game: {}", game_id);
    Ok(())
}

/// Resets game stats
#[tauri::command]
pub fn reset_stats(app_handle: AppHandle, game_id: String) -> CmdResult<()> {
    debug!("Resetting stats for game {}", game_id);
    let store = GamesStore::new(&app_handle).context("Error happened while accessing store")?;

    store
        .reset_stats(&game_id)
        .context("Error happened while resetting stats")?;

    info!("Successfully reset stats for game: {}", game_id);
    Ok(())
}

/// Updates the exe path of a game
#[tauri::command]
pub fn update_exe(app_handle: AppHandle, game_id: String, new_exe_path: String) -> CmdResult<()> {
    info!("Updating exe path for game {}: {}", game_id, new_exe_path);
    let store = GamesStore::new(&app_handle).context("Error happened while accessing store")?;

    store
        .update_game(&game_id, |g| g.exe_file_path = new_exe_path)
        .context("Error happened while updating exe")?;

    info!("Successfully updated exe path for game: {}", game_id);
    Ok(())
}

/// Updates the process path of a game
#[tauri::command]
pub fn update_process(
    app_handle: AppHandle,
    game_id: String,
    new_process_path: String,
) -> CmdResult<()> {
    info!(
        "Updating process path for game {}: {}",
        game_id, new_process_path
    );
    let store = GamesStore::new(&app_handle).context("Error happened while accessing store")?;

    store
        .update_game(&game_id, |g| g.process_file_path = new_process_path)
        .context("Error happened while updating process path")?;

    info!("Successfully updated process path for game: {}", game_id);
    Ok(())
}

/// Saves game notes to disk
#[tauri::command]
pub fn set_game_notes(app_handle: AppHandle, game_id: String, notes: String) -> CmdResult<()> {
    debug!(
        "Setting notes for game {}: {} characters",
        game_id,
        notes.len()
    );
    let store = GamesStore::new(&app_handle).context("Error happened while accessing store")?;

    store
        .update_game(&game_id, |g| g.notes = notes)
        .context("Error happened while setting notes")?;

    info!("Successfully set notes for game: {}", game_id);
    Ok(())
}

/// Sets the characters of an already saved game
#[tauri::command]
pub async fn set_characters(app_handle: AppHandle, game_id: String) -> CmdResult<()> {
    info!("Setting characters for game: {}", game_id);
    let chars = Vndb::get_vn_characters(&game_id)
        .await
        .context(format!("Error fetching characters for game {}", game_id))?;
    debug!("Found {} characters for game {}", chars.len(), game_id);

    let mut characters: Vec<Character> = Vec::new();

    for char in chars {
        debug!("Processing character: {} (ID: {})", char.name, char.id);
        let path = match char.image {
            Some(p) => {
                debug!("Saving character image for {} ({})", char.name, p.url);
                Some(
                    util::save_image(&app_handle, &p.url, None)
                        .await
                        .context("Error happened while saving image")?,
                )
            }
            None => {
                debug!("No image found for character: {}", char.name);
                None
            }
        };

        characters.push(Character {
            id: char.id,
            en_name: char.name,
            og_name: char.original,
            image_url: path,
        });
    }

    let store = GamesStore::new(&app_handle).context("Error happened while accessing store")?;

    let characters_len = characters.len();
    store
        .update_game(&game_id, |g| g.characters = Some(characters))
        .context("Error happened while saving characters")?;

    info!(
        "Successfully set {} characters for game: {}",
        characters_len, game_id
    );
    Ok(())
}

/// Gets all categories as an array
#[tauri::command]
pub fn get_categories(app_handle: AppHandle) -> CmdResult<Categories> {
    debug!("Getting all categories");
    let store =
        CategoriesStore::new(&app_handle).context("Error happened while accessing store")?;

    let categories = store.get_all().context("Couldn't get categories")?;

    debug!("Successfully loaded {} categories", categories.len());
    Ok(categories)
}

/// Sets categories from an array
#[tauri::command]
pub fn set_categories(app_handle: AppHandle, categories: Categories) -> CmdResult<()> {
    info!("Setting {} categories", categories.len());
    let store =
        CategoriesStore::new(&app_handle).context("Error happened while accessing store")?;

    let categories_len = categories.len();
    store
        .set(categories)
        .context("Error happened while setting categories")?;

    info!("Successfully set {} categories", categories_len);
    Ok(())
}

/// Gets all categories as an array
#[tauri::command]
pub fn get_selected_categories(app_handle: AppHandle) -> CmdResult<Categories> {
    debug!("Getting all selected categories");
    let store =
        CategoriesStore::new(&app_handle).context("Error happened while accessing store")?;

    let categories = store.get_selected().context("Couldn't get categories")?;

    debug!(
        "Successfully loaded {} selected categories",
        categories.len()
    );
    Ok(categories)
}

/// Sets categories from an array
#[tauri::command]
pub fn set_selected_categories(app_handle: AppHandle, categories: Categories) -> CmdResult<()> {
    info!("Setting {} selected categories", categories.len());
    let store =
        CategoriesStore::new(&app_handle).context("Error happened while accessing store")?;

    let categories_len = categories.len();
    store
        .set_selected(categories)
        .context("Error happened while setting categories")?;

    info!("Successfully set {} selected categories", categories_len);
    Ok(())
}

/// Sets categories of a game from an array
#[tauri::command]
pub fn set_game_categories(
    app_handle: AppHandle,
    game_id: String,
    categories: Categories,
) -> CmdResult<()> {
    info!(
        "Setting {} categories for game: {}",
        categories.len(),
        game_id
    );
    let store = GamesStore::new(&app_handle).context("Error happened while accessing store")?;

    let categories_len = categories.len();
    store
        .update_game(&game_id, |g| g.categories = categories)
        .context("Error happened while setting categories")?;

    info!(
        "Successfully set {} categories for game: {}",
        categories_len, game_id
    );
    Ok(())
}

/// Gets theme settings from storage
#[tauri::command]
pub fn get_theme_settings(app_handle: AppHandle) -> CmdResult<ThemeSettings> {
    let state = app_handle.state::<Mutex<AppState>>();
    let lock = state
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock state"))?;
    Ok(lock.settings.theme_settings.clone())
}

/// Saves theme settings to storage
#[tauri::command]
pub fn set_theme_settings(app_handle: AppHandle, theme_settings: ThemeSettings) -> CmdResult<()> {
    let state = app_handle.state::<Mutex<AppState>>();
    let mut lock = state
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock state"))?;
    lock.update_settings(&app_handle, |s| s.theme_settings = theme_settings)
        .context("Failed to update theme settings")?;
    Ok(())
}

/// Gets nsfw presence toggle status
#[tauri::command]
pub fn get_nsfw_presence_status(app_handle: AppHandle) -> CmdResult<bool> {
    let state = app_handle.state::<Mutex<AppState>>();
    let lock = state
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock state"))?;
    Ok(lock.settings.disable_presence_on_nsfw)
}

/// Saves theme settings to storage
#[tauri::command]
pub fn set_nsfw_presence_status(app_handle: AppHandle, to: bool) -> CmdResult<()> {
    let state = app_handle.state::<Mutex<AppState>>();
    let mut lock = state
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock state"))?;
    lock.update_settings(&app_handle, |s| s.disable_presence_on_nsfw = to)
        .context("Failed to update nsfw presence status")?;
    Ok(())
}

/// Gets sort order
#[tauri::command]
pub fn get_sort_order(app_handle: AppHandle) -> CmdResult<SortOrder> {
    let state = app_handle.state::<Mutex<AppState>>();
    let lock = state
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock state"))?;
    Ok(lock.settings.sort_order)
}

/// Saves theme settings to storage
#[tauri::command]
pub fn set_sort_order(app_handle: AppHandle, sort_order: SortOrder) -> CmdResult<()> {
    let state = app_handle.state::<Mutex<AppState>>();
    let mut lock = state
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock state"))?;
    lock.update_settings(&app_handle, |s| s.sort_order = sort_order)
        .context("Failed to update sort order")?;
    Ok(())
}

/// Gets show random picker
#[tauri::command]
pub fn get_show_random_picker(app_handle: AppHandle) -> CmdResult<bool> {
    let state = app_handle.state::<Mutex<AppState>>();
    let lock = state
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock state"))?;
    Ok(lock.settings.show_random_picker)
}

/// Saves show random picker setting to storage
#[tauri::command]
pub fn set_show_random_picker(app_handle: AppHandle, to: bool) -> CmdResult<()> {
    let state = app_handle.state::<Mutex<AppState>>();
    let mut lock = state
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock state"))?;
    lock.update_settings(&app_handle, |s| s.show_random_picker = to)
        .context("Failed to update show random picker")?;
    Ok(())
}

/// Gets discord presence mode
#[tauri::command]
pub fn get_discord_presence_mode(app_handle: AppHandle) -> CmdResult<DiscordPresenceMode> {
    let state = app_handle.state::<Mutex<AppState>>();
    let lock = state
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock state"))?;
    Ok(lock.settings.discord_presence_mode)
}

/// Saves Discord presence mode setting to storage
#[tauri::command]
pub fn set_discord_presence_mode(app_handle: AppHandle, to: DiscordPresenceMode) -> CmdResult<()> {
    let state = app_handle.state::<Mutex<AppState>>();
    let mut lock = state
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock state"))?;

    lock.update_settings(&app_handle, |s| s.discord_presence_mode = to)
        .context("Failed to update discord presence mode")?;

    if let Some(presence) = lock.presence.as_mut() {
        presence.set_mode(to);
    }

    Ok(())
}

/// Gets playtime mode
#[tauri::command]
pub fn get_playtime_mode(app_handle: AppHandle) -> CmdResult<PlaytimeMode> {
    let state = app_handle.state::<Mutex<AppState>>();
    let lock = state
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock state"))?;
    Ok(lock.settings.playtime_mode)
}

/// Saves new playtime mode to disk
#[tauri::command]
pub fn set_playtime_mode(app_handle: AppHandle, to: PlaytimeMode) -> CmdResult<()> {
    let state = app_handle.state::<Mutex<AppState>>();
    let mut lock = state
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock state"))?;
    lock.update_settings(&app_handle, |s| s.playtime_mode = to)
        .context("Failed to update playtime mode")?;
    Ok(())
}

#[tauri::command]
pub fn get_use_jp_for_title_time(app_handle: AppHandle) -> CmdResult<bool> {
    let state = app_handle.state::<Mutex<AppState>>();
    let lock = state
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock state"))?;
    Ok(lock.settings.use_jp_for_title_time)
}

#[tauri::command]
pub fn set_use_jp_for_title_time(app_handle: AppHandle, to: bool) -> CmdResult<()> {
    let state = app_handle.state::<Mutex<AppState>>();
    let mut lock = state
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock state"))?;
    lock.update_settings(&app_handle, |s| s.use_jp_for_title_time = to)
        .context("Failed to update use jp for title time")?;
    Ok(())
}

#[tauri::command]
pub fn get_hide_nsfw_images(app_handle: AppHandle) -> CmdResult<bool> {
    let state = app_handle.state::<Mutex<AppState>>();
    let lock = state
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock state"))?;
    Ok(lock.settings.hide_nsfw_images)
}

#[tauri::command]
pub fn set_hide_nsfw_images(app_handle: AppHandle, to: bool) -> CmdResult<()> {
    let state = app_handle.state::<Mutex<AppState>>();
    let mut lock = state
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock state"))?;
    lock.update_settings(&app_handle, |s| s.hide_nsfw_images = to)
        .context("Failed to update hide nsfw images")?;
    Ok(())
}

/// Gets the Jiten API base URL
#[tauri::command]
pub fn get_jiten_base_url(app_handle: AppHandle) -> CmdResult<String> {
    let state = app_handle.state::<Mutex<AppState>>();
    let lock = state
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock state"))?;
    Ok(lock.settings.jiten_base_url.clone())
}

/// Sets the Jiten API base URL
#[tauri::command]
pub fn set_jiten_base_url(app_handle: AppHandle, url: String) -> CmdResult<()> {
    let state = app_handle.state::<Mutex<AppState>>();
    let mut lock = state
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock state"))?;
    lock.update_settings(&app_handle, |s| s.jiten_base_url = url)
        .context("Failed to update jiten base url")?;
    Ok(())
}
