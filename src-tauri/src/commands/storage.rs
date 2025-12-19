use crate::{
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
    AppState,
};
use log::{debug, error, info};
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

    store
        .update_game(&game_id, |g| g.is_pinned = !g.is_pinned)
        .map_err(|e| {
            error!("Error toggling pin for game {}: {:?}", game_id, e);
            "Error happened while toggling pin"
        })?;

    info!("Successfully toggled pin state for game: {}", game_id);
    Ok(())
}

/// Resets game stats
#[tauri::command]
pub fn reset_stats(app_handle: AppHandle, game_id: String) -> Result<(), String> {
    debug!("Resetting stats for game {}", game_id);
    let store = GamesStore::new(&app_handle).map_err(|e| {
        error!(
            "Error accessing games store for resetting stats of {}: {:?}",
            game_id, e
        );
        "Error happened while accessing store"
    })?;

    store.reset_stats(&game_id).map_err(|e| {
        error!("Error resetting stats for game {}: {:?}", game_id, e);
        "Error happened while resetting stats"
    })?;

    info!("Successfully reset stats for game: {}", game_id);
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
        .update_game(&game_id, |g| g.exe_file_path = new_exe_path)
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
        .update_game(&game_id, |g| g.process_file_path = new_process_path)
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

    store
        .update_game(&game_id, |g| g.notes = notes)
        .map_err(|e| {
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
    store
        .update_game(&game_id, |g| g.characters = Some(characters))
        .map_err(|e| {
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
    store
        .update_game(&game_id, |g| g.categories = categories)
        .map_err(|e| {
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
    let state = app_handle.state::<Mutex<AppState>>();
    let lock = state.lock().map_err(|_| "Failed to lock state")?;
    Ok(lock.settings.theme_settings.clone())
}

/// Saves theme settings to storage
#[tauri::command]
pub fn set_theme_settings(
    app_handle: AppHandle,
    theme_settings: ThemeSettings,
) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppState>>();
    let mut lock = state.lock().map_err(|_| "Failed to lock state")?;
    lock.update_settings(&app_handle, |s| s.theme_settings = theme_settings)
        .map_err(|e| e.to_string())
}

/// Gets nsfw presence toggle status
#[tauri::command]
pub fn get_nsfw_presence_status(app_handle: AppHandle) -> Result<bool, String> {
    let state = app_handle.state::<Mutex<AppState>>();
    let lock = state.lock().map_err(|_| "Failed to lock state")?;
    Ok(lock.settings.disable_presence_on_nsfw)
}

/// Saves theme settings to storage
#[tauri::command]
pub fn set_nsfw_presence_status(app_handle: AppHandle, to: bool) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppState>>();
    let mut lock = state.lock().map_err(|_| "Failed to lock state")?;
    lock.update_settings(&app_handle, |s| s.disable_presence_on_nsfw = to)
        .map_err(|e| e.to_string())
}

/// Gets sort order
#[tauri::command]
pub fn get_sort_order(app_handle: AppHandle) -> Result<SortOrder, String> {
    let state = app_handle.state::<Mutex<AppState>>();
    let lock = state.lock().map_err(|_| "Failed to lock state")?;
    Ok(lock.settings.sort_order)
}

/// Saves theme settings to storage
#[tauri::command]
pub fn set_sort_order(app_handle: AppHandle, sort_order: SortOrder) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppState>>();
    let mut lock = state.lock().map_err(|_| "Failed to lock state")?;
    lock.update_settings(&app_handle, |s| s.sort_order = sort_order)
        .map_err(|e| e.to_string())
}

/// Gets show random picker
#[tauri::command]
pub fn get_show_random_picker(app_handle: AppHandle) -> Result<bool, String> {
    let state = app_handle.state::<Mutex<AppState>>();
    let lock = state.lock().map_err(|_| "Failed to lock state")?;
    Ok(lock.settings.show_random_picker)
}

/// Saves show random picker setting to storage
#[tauri::command]
pub fn set_show_random_picker(app_handle: AppHandle, to: bool) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppState>>();
    let mut lock = state.lock().map_err(|_| "Failed to lock state")?;
    lock.update_settings(&app_handle, |s| s.show_random_picker = to)
        .map_err(|e| e.to_string())
}

/// Gets discord presence mode
#[tauri::command]
pub fn get_discord_presence_mode(app_handle: AppHandle) -> Result<DiscordPresenceMode, String> {
    let state = app_handle.state::<Mutex<AppState>>();
    let lock = state.lock().map_err(|_| "Failed to lock state")?;
    Ok(lock.settings.discord_presence_mode)
}

/// Saves Discord presence mode setting to storage
#[tauri::command]
pub fn set_discord_presence_mode(
    app_handle: AppHandle,
    to: DiscordPresenceMode,
) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppState>>();
    let mut lock = state.lock().map_err(|_| "Failed to lock state")?;

    lock.update_settings(&app_handle, |s| s.discord_presence_mode = to)
        .map_err(|e| e.to_string())?;

    if let Some(presence) = lock.presence.as_mut() {
        presence.set_mode(to);
    }

    Ok(())
}

/// Gets playtime mode
#[tauri::command]
pub fn get_playtime_mode(app_handle: AppHandle) -> Result<PlaytimeMode, String> {
    let state = app_handle.state::<Mutex<AppState>>();
    let lock = state.lock().map_err(|_| "Failed to lock state")?;
    Ok(lock.settings.playtime_mode)
}

/// Saves new playtime mode to disk
#[tauri::command]
pub fn set_playtime_mode(app_handle: AppHandle, to: PlaytimeMode) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppState>>();
    let mut lock = state.lock().map_err(|_| "Failed to lock state")?;
    lock.update_settings(&app_handle, |s| s.playtime_mode = to)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_use_jp_for_title_time(app_handle: AppHandle) -> Result<bool, String> {
    let state = app_handle.state::<Mutex<AppState>>();
    let lock = state.lock().map_err(|_| "Failed to lock state")?;
    Ok(lock.settings.use_jp_for_title_time)
}

#[tauri::command]
pub fn set_use_jp_for_title_time(app_handle: AppHandle, to: bool) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppState>>();
    let mut lock = state.lock().map_err(|_| "Failed to lock state")?;
    lock.update_settings(&app_handle, |s| s.use_jp_for_title_time = to)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_hide_nsfw_images(app_handle: AppHandle) -> Result<bool, String> {
    let state = app_handle.state::<Mutex<AppState>>();
    let lock = state.lock().map_err(|_| "Failed to lock state")?;
    Ok(lock.settings.hide_nsfw_images)
}

#[tauri::command]
pub fn set_hide_nsfw_images(app_handle: AppHandle, to: bool) -> Result<(), String> {
    let state = app_handle.state::<Mutex<AppState>>();
    let mut lock = state.lock().map_err(|_| "Failed to lock state")?;
    lock.update_settings(&app_handle, |s| s.hide_nsfw_images = to)
        .map_err(|e| e.to_string())
}
