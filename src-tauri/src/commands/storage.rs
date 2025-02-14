use serde::{Deserialize, Serialize};
#[cfg(windows)]
use windows_icons;

use crate::{
    services::{
        store::{Categories, CategoriesStore, Character, Game, Games, GamesStore},
        vndb::Vndb,
    },
    util::{self},
};
use tauri::AppHandle;

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
    let _ = util::save_image(&app_handle, &game.image_url)
        .await
        .map_err(|_| "Error happened while saving image")?;

    #[cfg(windows)]
    {
        let icon = windows_icons::get_icon_by_path(&game.exe_file_path);
        let icon_path = format!(
            "{}.icon.png",
            path.to_str().ok_or("Couldn't convert path to string")?
        );

        dbg!(&icon_path);

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
            let path = util::save_image(&app_handle, &char.image.url)
                .await
                .map_err(|_| "Error happened while saving image")?;

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
        .map_err(|_| "Error happened while deleting game")?;

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
        .edit_exe(&game_id, &new_exe_path)
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
        .edit_process(&game_id, &new_process_path)
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
