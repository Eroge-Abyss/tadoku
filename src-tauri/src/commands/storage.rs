use crate::{
    services::{
        discord::DiscordPresenceMode,
        stores::{
            categories::{Categories, CategoriesStore},
            games::{Character, Game, Games, GamesStore},
            settings::{PlaytimeMode, SettingsStore, SortOrder, ThemeSettings},
        },
        vndb::Vndb,
    },
    util::{self},
    AppState,
};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{AppHandle, Manager};
#[cfg(windows)]
use windows_icons;

#[derive(Debug, Serialize, Deserialize)]
pub struct Options {
    include_characters: bool,
}

/// Saves a game to the local storage.
///
/// **NOTE**: This function downloads the image from the provided URL, saves it locally,
/// and updates the game data in the JSON store.
#[tauri::command]
pub async fn save_game(
    app_handle: AppHandle,
    game_id: String,
    mut game: Game,
    options: Options,
) -> Result<(), String> {
    info!("Saving game: {} ({})", game.title, game_id);
    debug!(
        "Game save options - include_characters: {}",
        options.include_characters
    );

    let _path = util::save_image(&app_handle, &game.image_url)
        .await
        .map_err(|e| {
            error!("Error saving image for game {}: {:?}", game_id, e);
            "Error happened while saving image"
        })?;

    debug!("Successfully saved game image for {}", game_id);

    #[cfg(windows)]
    {
        debug!("Extracting and saving icon for game {}", game_id);
        let icon = windows_icons::get_icon_by_path(&game.exe_file_path);
        let icon_path = format!("{}.icon.png", _path);
        icon.save(&icon_path).map_err(|e| {
            error!("Error saving icon for game {}: {:?}", game_id, e);
            "Error happened while saving image"
        })?;

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

    game.characters = if options.include_characters {
        info!("Fetching characters for game {}", game_id);
        let chars = Vndb::get_vn_characters(&game_id).await.map_err(|e| {
            error!("Error fetching characters for game {}: {}", game_id, e);
            e
        })?;
        debug!("Found {} characters for game {}", chars.len(), game_id);

        let mut new_chars: Vec<Character> = Vec::new();

        for char in chars {
            debug!("Processing character: {} (ID: {})", char.name, char.id);
            let path = match char.image {
                Some(p) => {
                    debug!("Saving character image for {} ({})", char.name, p.url);
                    Some(util::save_image(&app_handle, &p.url).await.map_err(|e| {
                        error!("Error saving character image for {}: {:?}", char.name, e);
                        "Error happened while saving image"
                    })?)
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
        Some(new_chars)
    } else {
        debug!("Skipping character fetching for game {}", game_id);
        None
    };

    let store = GamesStore::new(&app_handle).map_err(|e| {
        error!("Error accessing games store for {}: {:?}", game_id, e);
        "Error happened while accessing store"
    })?;

    store.save(game_id.clone(), game).map_err(|e| {
        error!("Error saving game {} to store: {:?}", game_id, e);
        "Error happened while saving game"
    })?;

    info!("Successfully saved game: {}", game_id);
    Ok(())
}

/// Loads all games from JSON storage
#[tauri::command]
pub fn load_games(app_handle: AppHandle) -> Result<Games, String> {
    debug!("Loading all games from storage");
    let store = GamesStore::new(&app_handle).map_err(|e| {
        error!("Error accessing games store: {:?}", e);
        "Error happened while accessing store"
    })?;

    let games_data = store.get_all().map_err(|e| {
        error!("Error loading games from store: {:?}", e);
        "Error happened while getting games"
    })?;

    info!(
        "Successfully loaded {} games from storage",
        games_data.len()
    );
    Ok(games_data)
}

/// Deletes a game from JSON storage
#[tauri::command]
pub fn delete_game(app_handle: AppHandle, game_id: String) -> Result<(), String> {
    info!("Deleting game: {}", game_id);
    let store = GamesStore::new(&app_handle).map_err(|e| {
        error!(
            "Error accessing games store for deletion of {}: {:?}",
            game_id, e
        );
        "Error happened while accessing store"
    })?;

    store.delete(&game_id).map_err(|e| {
        error!("Error deleting game {} from store: {:?}", game_id, e);
        "Error happened while deleting game"
    })?;

    info!("Successfully deleted game: {}", game_id);
    Ok(())
}

/// Toggles the pinned state of a game
#[tauri::command]
pub fn toggle_pin(app_handle: AppHandle, game_id: String) -> Result<(), String> {
    debug!("Toggling pin state for game: {}", game_id);
    let store = GamesStore::new(&app_handle).map_err(|e| {
        error!(
            "Error accessing games store for pin toggle of {}: {:?}",
            game_id, e
        );
        "Error happened while accessing store"
    })?;

    store.toggle_pin(&game_id).map_err(|e| {
        error!("Error toggling pin for game {}: {:?}", game_id, e);
        "Error happened while toggling pin"
    })?;

    info!("Successfully toggled pin state for game: {}", game_id);
    Ok(())
}

/// Updates the exe path of a game
#[tauri::command]
pub fn update_exe(
    app_handle: AppHandle,
    game_id: String,
    new_exe_path: String,
) -> Result<(), String> {
    info!("Updating exe path for game {}: {}", game_id, new_exe_path);
    let store = GamesStore::new(&app_handle).map_err(|e| {
        error!(
            "Error accessing games store for exe update of {}: {:?}",
            game_id, e
        );
        "Error happened while accessing store"
    })?;

    store
        .update_exe_path(&game_id, &new_exe_path)
        .map_err(|e| {
            error!("Error updating exe path for game {}: {:?}", game_id, e);
            "Error happened while updating exe"
        })?;

    info!("Successfully updated exe path for game: {}", game_id);
    Ok(())
}

/// Updates the process path of a game
#[tauri::command]
pub fn update_process(
    app_handle: AppHandle,
    game_id: String,
    new_process_path: String,
) -> Result<(), String> {
    info!(
        "Updating process path for game {}: {}",
        game_id, new_process_path
    );
    let store = GamesStore::new(&app_handle).map_err(|e| {
        error!(
            "Error accessing games store for process update of {}: {:?}",
            game_id, e
        );
        "Error happened while accessing store"
    })?;

    store
        .update_process_path(&game_id, &new_process_path)
        .map_err(|e| {
            error!("Error updating process path for game {}: {:?}", game_id, e);
            "Error happened while updating process path"
        })?;

    info!("Successfully updated process path for game: {}", game_id);
    Ok(())
}

/// Saves game notes to disk
#[tauri::command]
pub fn set_game_notes(app_handle: AppHandle, game_id: String, notes: String) -> Result<(), String> {
    debug!(
        "Setting notes for game {}: {} characters",
        game_id,
        notes.len()
    );
    let store = GamesStore::new(&app_handle).map_err(|e| {
        error!(
            "Error accessing games store for notes update of {}: {:?}",
            game_id, e
        );
        "Error happened while accessing store"
    })?;

    store.set_notes(&game_id, &notes).map_err(|e| {
        error!("Error setting notes for game {}: {:?}", game_id, e);
        "Error happened while setting notes"
    })?;

    info!("Successfully set notes for game: {}", game_id);
    Ok(())
}

/// Sets the characters of an already saved game
#[tauri::command]
pub async fn set_characters(app_handle: AppHandle, game_id: String) -> Result<(), String> {
    info!("Setting characters for game: {}", game_id);
    let chars = Vndb::get_vn_characters(&game_id).await.map_err(|e| {
        error!("Error fetching characters for game {}: {}", game_id, e);
        e
    })?;
    debug!("Found {} characters for game {}", chars.len(), game_id);

    let mut characters: Vec<Character> = Vec::new();

    for char in chars {
        debug!("Processing character: {} (ID: {})", char.name, char.id);
        let path = match char.image {
            Some(p) => {
                debug!("Saving character image for {} ({})", char.name, p.url);
                Some(util::save_image(&app_handle, &p.url).await.map_err(|e| {
                    error!("Error saving character image for {}: {:?}", char.name, e);
                    "Error happened while saving image"
                })?)
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

    let store = GamesStore::new(&app_handle).map_err(|e| {
        error!(
            "Error accessing games store for character update of {}: {:?}",
            game_id, e
        );
        "Error happened while accessing store"
    })?;

    let characters_len = characters.len();
    store.set_characters(&game_id, characters).map_err(|e| {
        error!("Error setting characters for game {}: {:?}", game_id, e);
        "Error happened while saving characters"
    })?;

    info!(
        "Successfully set {} characters for game: {}",
        characters_len, game_id
    );
    Ok(())
}

/// Gets all categories as an array
#[tauri::command]
pub fn get_categories(app_handle: AppHandle) -> Result<Categories, String> {
    debug!("Getting all categories");
    let store = CategoriesStore::new(&app_handle).map_err(|e| {
        error!("Error accessing categories store: {:?}", e);
        "Error happened while accessing store"
    })?;

    let categories = store.get_all().map_err(|e| {
        error!("Error loading categories: {:?}", e);
        "Couldn't get categories"
    })?;

    debug!("Successfully loaded {} categories", categories.len());
    Ok(categories)
}

/// Sets categories from an array
#[tauri::command]
pub fn set_categories(app_handle: AppHandle, categories: Categories) -> Result<(), String> {
    info!("Setting {} categories", categories.len());
    let store = CategoriesStore::new(&app_handle).map_err(|e| {
        error!("Error accessing categories store: {:?}", e);
        "Error happened while accessing store"
    })?;

    let categories_len = categories.len();
    store.set(categories).map_err(|e| {
        error!("Error setting categories: {:?}", e);
        "Error happened while setting categories"
    })?;

    info!("Successfully set {} categories", categories_len);
    Ok(())
}

/// Gets all categories as an array
#[tauri::command]
pub fn get_selected_categories(app_handle: AppHandle) -> Result<Categories, String> {
    debug!("Getting all selected categories");
    let store = CategoriesStore::new(&app_handle).map_err(|e| {
        error!("Error accessing categories store: {:?}", e);
        "Error happened while accessing store"
    })?;

    let categories = store.get_selected().map_err(|e| {
        error!("Error loading categories: {:?}", e);
        "Couldn't get categories"
    })?;

    debug!(
        "Successfully loaded {} selected categories",
        categories.len()
    );
    Ok(categories)
}

/// Sets categories from an array
#[tauri::command]
pub fn set_selected_categories(
    app_handle: AppHandle,
    categories: Categories,
) -> Result<(), String> {
    info!("Setting {} selected categories", categories.len());
    let store = CategoriesStore::new(&app_handle).map_err(|e| {
        error!("Error accessing categories store: {:?}", e);
        "Error happened while accessing store"
    })?;

    let categories_len = categories.len();
    store.set_selected(categories).map_err(|e| {
        error!("Error setting categories: {:?}", e);
        "Error happened while setting categories"
    })?;

    info!("Successfully set {} selected categories", categories_len);
    Ok(())
}

/// Sets categories of a game from an array
#[tauri::command]
pub fn set_game_categories(
    app_handle: AppHandle,
    game_id: String,
    categories: Categories,
) -> Result<(), String> {
    info!(
        "Setting {} categories for game: {}",
        categories.len(),
        game_id
    );
    let store = GamesStore::new(&app_handle).map_err(|e| {
        error!(
            "Error accessing games store for category update of {}: {:?}",
            game_id, e
        );
        "Error happened while accessing store"
    })?;

    let categories_len = categories.len();
    store.set_categories(&game_id, categories).map_err(|e| {
        error!("Error setting categories for game {}: {:?}", game_id, e);
        "Error happened while setting categories"
    })?;

    info!(
        "Successfully set {} categories for game: {}",
        categories_len, game_id
    );
    Ok(())
}

/// Gets theme settings from storage
#[tauri::command]
pub fn get_theme_settings(app_handle: AppHandle) -> Result<ThemeSettings, String> {
    debug!("Getting theme settings");
    let store = SettingsStore::new(&app_handle).map_err(|e| {
        error!("Error accessing settings store: {:?}", e);
        "Error happened while accessing store"
    })?;

    let theme_settings = store.get_theme_settings().map_err(|e| {
        error!("Error loading theme settings: {:?}", e);
        "Couldn't get theme settings"
    })?;

    debug!("Successfully loaded theme settings");
    Ok(theme_settings)
}

/// Saves theme settings to storage
#[tauri::command]
pub fn set_theme_settings(
    app_handle: AppHandle,
    theme_settings: ThemeSettings,
) -> Result<(), String> {
    info!("Setting theme settings");
    let store = SettingsStore::new(&app_handle).map_err(|e| {
        error!("Error accessing settings store: {:?}", e);
        "Error happened while accessing store"
    })?;

    store.set_theme_settings(theme_settings).map_err(|e| {
        error!("Error setting theme settings: {:?}", e);
        "Error happened while setting theme settings"
    })?;

    info!("Successfully set theme settings");
    Ok(())
}

/// Gets nsfw presence toggle status
#[tauri::command]
pub fn get_nsfw_presence_status(app_handle: AppHandle) -> Result<bool, String> {
    debug!("Getting NSFW presence status");
    let store = SettingsStore::new(&app_handle).map_err(|e| {
        error!("Error accessing settings store: {:?}", e);
        "Error happened while accessing store"
    })?;

    let status = store.get_presence_on_nsfw().map_err(|e| {
        error!("Error loading NSFW presence status: {:?}", e);
        "Couldn't get theme settings"
    })?;

    debug!("NSFW presence status: {}", status);
    Ok(status)
}

/// Saves theme settings to storage
#[tauri::command]
pub fn set_nsfw_presence_status(app_handle: AppHandle, to: bool) -> Result<(), String> {
    info!("Setting NSFW presence status to: {}", to);
    let store = SettingsStore::new(&app_handle).map_err(|e| {
        error!("Error accessing settings store: {:?}", e);
        "Error happened while accessing store"
    })?;

    store.set_presence_on_nsfw(to).map_err(|e| {
        error!("Error setting NSFW presence status: {:?}", e);
        "Error happened while setting theme settings"
    })?;

    let binding = app_handle.state::<Mutex<AppState>>();

    let mut app_state = binding.lock().map_err(|e| {
        error!("Cannot acquire state lock for NSFW presence: {:?}", e);
        "Cannot acquire state lock"
    })?;

    app_state.config.disable_presence_on_nsfw = to;

    info!("Successfully set NSFW presence status to: {}", to);
    Ok(())
}

/// Gets sort order
#[tauri::command]
pub fn get_sort_order(app_handle: AppHandle) -> Result<SortOrder, String> {
    debug!("Getting sort order");
    let store = SettingsStore::new(&app_handle).map_err(|e| {
        error!("Error accessing settings store: {:?}", e);
        "Error happened while accessing store"
    })?;

    let sort_order = store.get_sort_order().map_err(|e| {
        error!("Error loading sort order: {:?}", e);
        "Couldn't get sort order"
    })?;

    debug!("Sort order loaded successfully");
    Ok(sort_order)
}

/// Saves theme settings to storage
#[tauri::command]
pub fn set_sort_order(app_handle: AppHandle, sort_order: SortOrder) -> Result<(), String> {
    info!("Setting sort order");
    let store = SettingsStore::new(&app_handle).map_err(|e| {
        error!("Error accessing settings store: {:?}", e);
        "Error happened while accessing store"
    })?;

    store.set_sort_order(sort_order).map_err(|e| {
        error!("Error setting sort order: {:?}", e);
        "Error happened while setting sort order"
    })?;

    info!("Successfully set sort order");
    Ok(())
}

/// Gets show random picker
#[tauri::command]
pub fn get_show_random_picker(app_handle: AppHandle) -> Result<bool, String> {
    debug!("Getting show random picker setting");
    let store = SettingsStore::new(&app_handle).map_err(|e| {
        error!("Error accessing settings store: {:?}", e);
        "Error happened while accessing store"
    })?;

    let show_picker = store.get_show_random_picker().map_err(|e| {
        error!("Error loading show random picker setting: {:?}", e);
        "Couldn't get show random picker"
    })?;

    debug!("Show random picker: {}", show_picker);
    Ok(show_picker)
}

/// Saves show random picker setting to storage
#[tauri::command]
pub fn set_show_random_picker(app_handle: AppHandle, to: bool) -> Result<(), String> {
    info!("Setting show random picker to: {}", to);
    let store = SettingsStore::new(&app_handle).map_err(|e| {
        error!("Error accessing settings store: {:?}", e);
        "Error happened while accessing store"
    })?;

    store.set_show_random_picker(to).map_err(|e| {
        error!("Error setting show random picker: {:?}", e);
        "Error happened while setting show random picker"
    })?;

    info!("Successfully set show random picker to: {}", to);
    Ok(())
}

/// Gets discord presence mode
#[tauri::command]
pub fn get_discord_presence_mode(app_handle: AppHandle) -> Result<DiscordPresenceMode, String> {
    debug!("Getting Discord presence mode");
    let store = SettingsStore::new(&app_handle).map_err(|e| {
        error!("Error accessing settings store: {:?}", e);
        "Error happened while accessing store"
    })?;

    let mode = store.get_discord_presence_mode().map_err(|e| {
        error!("Error loading Discord presence mode: {:?}", e);
        "Couldn't get discord presence mode"
    })?;

    debug!("Discord presence mode loaded successfully");
    Ok(mode)
}

/// Saves Discord presence mode setting to storage
#[tauri::command]
pub fn set_discord_presence_mode(
    app_handle: AppHandle,
    to: DiscordPresenceMode,
) -> Result<(), String> {
    info!("Setting Discord presence mode");
    let store = SettingsStore::new(&app_handle).map_err(|e| {
        error!("Error accessing settings store: {:?}", e);
        "Error happened while accessing store"
    })?;

    store.set_discord_presence_mode(to).map_err(|e| {
        error!("Error setting Discord presence mode: {:?}", e);
        "Error happened while setting discord presence mode"
    })?;

    let binding = app_handle.state::<Mutex<AppState>>();

    let mut app_state = binding.lock().map_err(|e| {
        error!(
            "Cannot acquire state lock for Discord presence mode: {:?}",
            e
        );
        "Cannot acquire state lock"
    })?;

    if let Some(presence) = app_state.presence.as_mut() {
        debug!("Updating Discord presence mode in runtime state");
        presence.set_mode(to);
    } else {
        warn!("No Discord presence instance found in app state");
    }

    info!("Successfully set Discord presence mode");
    Ok(())
}

/// Gets playtime mode
#[tauri::command]
pub fn get_playtime_mode(app_handle: AppHandle) -> Result<PlaytimeMode, String> {
    debug!("Getting playtime mode");
    let store = SettingsStore::new(&app_handle).map_err(|e| {
        error!("Error accessing settings store: {:?}", e);
        "Error happened while accessing store"
    })?;

    let mode = store.get_playtime_mode().map_err(|e| {
        error!("Error loading playtime mode: {:?}", e);
        "Couldn't get playtime mode"
    })?;

    debug!("Playtime mode loaded successfully");
    Ok(mode)
}

/// Saves new playtime mode to disk
#[tauri::command]
pub fn set_playtime_mode(app_handle: AppHandle, to: PlaytimeMode) -> Result<(), String> {
    info!("Setting playtime mode");
    let store = SettingsStore::new(&app_handle).map_err(|e| {
        error!("Error accessing settings store: {:?}", e);
        "Error happened while accessing store"
    })?;

    store.set_playtime_mode(to).map_err(|e| {
        error!("Error setting playtime mode: {:?}", e);
        "Error happened while setting playtime mode"
    })?;

    info!("Successfully set playtime mode");
    Ok(())
}

#[tauri::command]
pub fn get_use_jp_for_title_time(app_handle: AppHandle) -> Result<bool, String> {
    info!("Getting use_jp_for_title_time setting");
    let store = SettingsStore::new(&app_handle).map_err(|e| {
        error!("Failed to create settings store: {:?}", e);
        e.to_string()
    })?;

    let setting = store.get_use_jp_for_title_time().map_err(|e| {
        error!("Failed to get use_jp_for_title_time setting: {:?}", e);
        e.to_string()
    })?;

    info!(
        "Successfully got use_jp_for_title_time setting: {}",
        setting
    );
    Ok(setting)
}

#[tauri::command]
pub fn set_use_jp_for_title_time(app_handle: AppHandle, to: bool) -> Result<(), String> {
    info!("Setting use_jp_for_title_time to: {}", to);
    let store = SettingsStore::new(&app_handle).map_err(|e| {
        error!("Failed to create settings store: {:?}", e);
        e.to_string()
    })?;

    store.set_use_jp_for_title_time(to).map_err(|e| {
        error!("Failed to set use_jp_for_title_time: {:?}", e);
        e.to_string()
    })?;

    info!("Successfully set use_jp_for_title_time to: {}", to);
    Ok(())
}
