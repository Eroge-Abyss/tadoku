use crate::util;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io::Cursor};
use tauri::{AppHandle, Manager};
use tauri_plugin_http::reqwest;
use tauri_plugin_store::StoreExt;

// TODO: String vs &str
#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    title: String,
    description: String,
    // Is a VNDB link when Game is an input, should be a local saved file url after saving
    image_url: String,
    exe_file_path: String,
    // Play time in seconds or smth
    play_time: u32,
}

type Games = HashMap<String, Game>;

/// Saves a game to the local storage.
///
/// **NOTE**: This function downloads the image from the provided URL, saves it locally,
/// and updates the game data in the JSON store.
#[tauri::command]
pub async fn save_game(
    app_handle: AppHandle,
    game_id: String,
    mut game: Game,
) -> Result<(), String> {
    let response = reqwest::get(&game.image_url)
        .await
        .map_err(|_| "Failed to fetch image")?;
    let path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|err| err.to_string())?
        .join("images")
        .join(util::extract_image(&game.image_url));

    let mut file = fs::File::create(&path).map_err(|err| err.to_string())?;
    let mut content = Cursor::new(response.bytes().await.map_err(|err| err.to_string())?);

    std::io::copy(&mut content, &mut file).map_err(|_| "Failed to download image")?;

    let store = app_handle.store("store.json").unwrap();
    let mut games_data: Games = store
        .get("gamesData")
        .map(|v| serde_json::from_value(v).expect("Value should be JSON"))
        .unwrap_or(HashMap::new());

    game.image_url = String::from(path.to_str().expect("Path should be a string"));

    // TODO: Make games with same ID throw error / overwrite
    games_data.insert(game_id, game);

    store.set("gamesData", serde_json::to_value(games_data).unwrap());

    Ok(())
}

/// Loads all games from JSON storage
#[tauri::command]
pub fn load_games(app_handle: AppHandle) -> Result<Games, String> {
    let store = app_handle
        .store("store.json")
        .map_err(|_| "Couldn't access games data")?;

    Ok(store
        .get("gamesData")
        .map(|v| serde_json::from_value(v).expect("Value should be JSON"))
        .unwrap_or(HashMap::new()))
}

/// Deletes a game from JSON storage
#[tauri::command]
pub fn delete_game(app_handle: AppHandle, game_id: String) -> Result<(), String> {
    let store = app_handle
        .store("store.json")
        .map_err(|_| "Couldn't access games data")?;

    let mut games_data: Games = store
        .get("gamesData")
        .map(|v| serde_json::from_value(v).expect("Value should be JSON"))
        .unwrap_or(HashMap::new());

    if let Some(game) = games_data.remove(&game_id) {
        let path = app_handle
            .path()
            .app_local_data_dir()
            .map_err(|_| "Error happened while getting path")?
            .join(game.image_url);

        fs::remove_file(path).map_err(|err| err.to_string())?;

        store.set(
            "gamesData",
            serde_json::to_value(games_data).map_err(|_| "Error happened while deleting game")?,
        );
    }

    Ok(())
}
