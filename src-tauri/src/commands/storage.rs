use crate::{
    services::store::{Game, Games, GamesStore},
    util::{self},
};
use std::{fs, io::Cursor};
use tauri::{AppHandle, Manager};
use tauri_plugin_http::reqwest;

/// Saves a game to the local storage.
///
/// **NOTE**: This function downloads the image from the provided URL, saves it locally,
/// and updates the game data in the JSON store.
#[tauri::command]
pub async fn save_game(app_handle: AppHandle, game_id: String, game: Game) -> Result<(), String> {
    let response = reqwest::get(&game.image_url)
        .await
        .map_err(|_| "Failed to fetch image")?;

    let base_path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|err| err.to_string())?;

    let path = util::construct_image_path(&base_path, &game.image_url);

    let mut file = fs::File::create(&path).map_err(|err| err.to_string())?;
    let mut content = Cursor::new(response.bytes().await.map_err(|err| err.to_string())?);

    std::io::copy(&mut content, &mut file).map_err(|_| "Failed to download image")?;

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
