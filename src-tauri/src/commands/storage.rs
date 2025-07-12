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
    let _path = util::save_image(&app_handle, &game.image_url)
        .await
        .map_err(|_| "Error happened while saving image")?;

    #[cfg(windows)]
    {
        let icon = windows_icons::get_icon_by_path(&game.exe_file_path);
        let icon_path = format!("{}.icon.png", _path);
        icon.save(&icon_path)
            .map_err(|_| "Error happened while saving image")?;

        game.icon_url = Some(icon_path);
    }

    #[cfg(not(windows))]
    {
        game.icon_url = None;
    }

    game.characters = if options.include_characters {
        let chars = Vndb::get_vn_characters(&game_id).await?;
        let mut new_chars: Vec<Character> = Vec::new();

        for char in chars {
            let path = match char.image {
                Some(p) => Some(
                    util::save_image(&app_handle, &p.url)
                        .await
                        .map_err(|_| "Error happened while saving image")?,
                ),
                None => None,
            };

            new_chars.push(Character {
                id: char.id,
                en_name: char.name,
                og_name: char.original,
                image_url: path,
            });
        }

        Some(new_chars)
    } else {
        None
    };

    let store = GamesStore::new(&app_handle).map_err(|_| "Error happened while accessing store")?;

    store
        .save(game_id, game)
        .map_err(|_| "Error happened while saving game")?;

    Ok(())
}

/// Loads all games from JSON storage
#[tauri::command]
pub fn load_games(app_handle: AppHandle) -> Result<Games, String> {
    let store = GamesStore::new(&app_handle).map_err(|_| "Error happened while accessing store")?;

    let games_data = store
        .get_all()
        .map_err(|_| "Error happened while getting games")?;

    Ok(games_data)
}

/// Deletes a game from JSON storage
#[tauri::command]
pub fn delete_game(app_handle: AppHandle, game_id: String) -> Result<(), String> {
    let store = GamesStore::new(&app_handle).map_err(|_| "Error happened while accessing store")?;

    store
        .delete(&game_id)
        .map_err(|_| "Error happened while deleting game")?;

    Ok(())
}

/// Toggles the pinned state of a game
#[tauri::command]
pub fn toggle_pin(app_handle: AppHandle, game_id: String) -> Result<(), String> {
    let store = GamesStore::new(&app_handle).map_err(|_| "Error happened while accessing store")?;

    store
        .toggle_pin(&game_id)
        .map_err(|_| "Error happened while toggling pin")?;

    Ok(())
}

/// Updates the exe path of a game
#[tauri::command]
pub fn update_exe(
    app_handle: AppHandle,
    game_id: String,
    new_exe_path: String,
) -> Result<(), String> {
    let store = GamesStore::new(&app_handle).map_err(|_| "Error happened while accessing store")?;

    store
        .update_exe_path(&game_id, &new_exe_path)
        .map_err(|_| "Error happened while updating exe")?;

    Ok(())
}

/// Updates the process path of a game
#[tauri::command]
pub fn update_process(
    app_handle: AppHandle,
    game_id: String,
    new_process_path: String,
) -> Result<(), String> {
    let store = GamesStore::new(&app_handle).map_err(|_| "Error happened while accessing store")?;

    store
        .update_process_path(&game_id, &new_process_path)
        .map_err(|_| "Error happened while updating process path")?;

    Ok(())
}

/// Gets all categories as an array
#[tauri::command]
pub fn get_categories(app_handle: AppHandle) -> Result<Categories, String> {
    let store =
        CategoriesStore::new(&app_handle).map_err(|_| "Error happened while accessing store")?;

    Ok(store.get_all().map_err(|_| "Couldn't get categories")?)
}

/// Sets categories from an array
#[tauri::command]
pub fn set_categories(app_handle: AppHandle, categories: Categories) -> Result<(), String> {
    let store =
        CategoriesStore::new(&app_handle).map_err(|_| "Error happened while accessing store")?;

    store
        .set(categories)
        .map_err(|_| "Error happened while setting categories")?;

    Ok(())
}

/// Sets categories of a game from an array
#[tauri::command]
pub fn set_game_categories(
    app_handle: AppHandle,
    game_id: String,
    categories: Categories,
) -> Result<(), String> {
    let store = GamesStore::new(&app_handle).map_err(|_| "Error happened while accessing store")?;

    store
        .set_categories(&game_id, categories)
        .map_err(|_| "Error happened while setting categories")?;

    Ok(())
}

/// Gets theme settings from storage
#[tauri::command]
pub fn get_theme_settings(app_handle: AppHandle) -> Result<ThemeSettings, String> {
    let store =
        SettingsStore::new(&app_handle).map_err(|_| "Error happened while accessing store")?;

    Ok(store
        .get_theme_settings()
        .map_err(|_| "Couldn't get theme settings")?)
}

/// Saves theme settings to storage
#[tauri::command]
pub fn set_theme_settings(
    app_handle: AppHandle,
    theme_settings: ThemeSettings,
) -> Result<(), String> {
    let store =
        SettingsStore::new(&app_handle).map_err(|_| "Error happened while accessing store")?;

    store
        .set_theme_settings(theme_settings)
        .map_err(|_| "Error happened while setting theme settings")?;

    Ok(())
}

/// Gets nsfw presence toggle status
#[tauri::command]
pub fn get_nsfw_presence_status(app_handle: AppHandle) -> Result<bool, String> {
    let store =
        SettingsStore::new(&app_handle).map_err(|_| "Error happened while accessing store")?;

    Ok(store
        .get_presence_on_nsfw()
        .map_err(|_| "Couldn't get theme settings")?)
}

/// Saves theme settings to storage
#[tauri::command]
pub fn set_nsfw_presence_status(app_handle: AppHandle, to: bool) -> Result<(), String> {
    let store =
        SettingsStore::new(&app_handle).map_err(|_| "Error happened while accessing store")?;

    store
        .set_presence_on_nsfw(to)
        .map_err(|_| "Error happened while setting theme settings")?;

    let binding = app_handle.state::<Mutex<AppState>>();

    let mut app_state = binding.lock().map_err(|_| "Cannot acquire state lock")?;

    app_state.config.disable_presence_on_nsfw = to;

    Ok(())
}

/// Sets the characters of an already saved game
#[tauri::command]
pub async fn set_characters(app_handle: AppHandle, game_id: String) -> Result<(), String> {
    let chars = Vndb::get_vn_characters(&game_id).await?;
    let mut characters: Vec<Character> = Vec::new();

    for char in chars {
        let path = match char.image {
            Some(p) => Some(
                util::save_image(&app_handle, &p.url)
                    .await
                    .map_err(|_| "Error happened while saving image")?,
            ),
            None => None,
        };

        characters.push(Character {
            id: char.id,
            en_name: char.name,
            og_name: char.original,
            image_url: path,
        });
    }

    let store = GamesStore::new(&app_handle).map_err(|_| "Error happened while accessing store")?;

    store
        .set_characters(&game_id, characters)
        .map_err(|_| "Error happened while saving characters")?;

    Ok(())
}

/// Gets sort order
#[tauri::command]
pub fn get_sort_order(app_handle: AppHandle) -> Result<SortOrder, String> {
    let store =
        SettingsStore::new(&app_handle).map_err(|_| "Error happened while accessing store")?;

    Ok(store
        .get_sort_order()
        .map_err(|_| "Couldn't get sort order")?)
}

/// Saves theme settings to storage
#[tauri::command]
pub fn set_sort_order(app_handle: AppHandle, sort_order: SortOrder) -> Result<(), String> {
    let store =
        SettingsStore::new(&app_handle).map_err(|_| "Error happened while accessing store")?;

    store
        .set_sort_order(sort_order)
        .map_err(|_| "Error happened while setting sort order")?;

    Ok(())
}

/// Gets show random picker
#[tauri::command]
pub fn get_show_random_picker(app_handle: AppHandle) -> Result<bool, String> {
    let store =
        SettingsStore::new(&app_handle).map_err(|_| "Error happened while accessing store")?;

    Ok(store
        .get_show_random_picker()
        .map_err(|_| "Couldn't get show random picker")?)
}

/// Saves show random picker setting to storage
#[tauri::command]
pub fn set_show_random_picker(app_handle: AppHandle, to: bool) -> Result<(), String> {
    let store =
        SettingsStore::new(&app_handle).map_err(|_| "Error happened while accessing store")?;

    store
        .set_show_random_picker(to)
        .map_err(|_| "Error happened while setting show random picker")?;

    Ok(())
}

/// Gets discord presence mode
#[tauri::command]
pub fn get_discord_presence_mode(app_handle: AppHandle) -> Result<DiscordPresenceMode, String> {
    let store =
        SettingsStore::new(&app_handle).map_err(|_| "Error happened while accessing store")?;

    Ok(store
        .get_discord_presence_mode()
        .map_err(|_| "Couldn't get discord presence mode")?)
}

/// Saves show random picker setting to storage
#[tauri::command]
pub fn set_discord_presence_mode(
    app_handle: AppHandle,
    to: DiscordPresenceMode,
) -> Result<(), String> {
    let store =
        SettingsStore::new(&app_handle).map_err(|_| "Error happened while accessing store")?;

    store
        .set_discord_presence_mode(to)
        .map_err(|_| "Error happened while setting discord presence mode")?;

    let binding = app_handle.state::<Mutex<AppState>>();

    let mut app_state = binding.lock().map_err(|_| "Cannot acquire state lock")?;

    if let Some(presence) = app_state.presence.as_mut() {
        presence.set_mode(to);
    };

    Ok(())
}

/// Gets playtime mode
#[tauri::command]
pub fn get_playtime_mode(app_handle: AppHandle) -> Result<PlaytimeMode, String> {
    let store =
        SettingsStore::new(&app_handle).map_err(|_| "Error happened while accessing store")?;

    Ok(store
        .get_playtime_mode()
        .map_err(|_| "Couldn't get playtime mode")?)
}

/// Saves new playtime mode to disk
#[tauri::command]
pub fn set_playtime_mode(app_handle: AppHandle, to: PlaytimeMode) -> Result<(), String> {
    let store =
        SettingsStore::new(&app_handle).map_err(|_| "Error happened while accessing store")?;

    store
        .set_playtime_mode(to)
        .map_err(|_| "Error happened while setting playtime mode")?;

    Ok(())
}
