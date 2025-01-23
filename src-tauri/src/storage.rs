// TODO
// Settle on a format to save data
// which is for now, VNDB data, play time when added
// {gamesData: []}

use std::io::Cursor;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tauri_plugin_http::reqwest;
use tauri_plugin_store::StoreExt;

// TODO: String vs &str
#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    id: String,
    title: String,
    description: String,
    // Is a VNDB link when Game is an input, should be a local saved file url after saving
    image_url: String,
    exe_file_path: String,
    // Play time in seconds or smth
    play_time: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GamesJson {
    games_data: Vec<Game>,
}

#[tauri::command]
pub async fn save_game(app_handle: AppHandle, mut game: Game) {
    // TODO: Improve this gayass code (error handling etc.)
    // Should convert the image_url to a local file
    // and save data in the json file
    // TODO: IT DOESN'T CREATE THE IMAGES FOLDER (check if it can be done using setup in lib)
    let response = reqwest::get(&game.image_url).await.unwrap();

    let path = app_handle
        .path()
        .app_local_data_dir()
        .unwrap()
        .join("images")
        // TODO: Deal with different image formats
        .join(format!("{}.{}", game.id, "jpg"));

    dbg!(&path);
    dbg!(&app_handle.path().app_local_data_dir());
    let mut file = std::fs::File::create(&path).unwrap();

    let mut content = Cursor::new(response.bytes().await.unwrap());
    std::io::copy(&mut content, &mut file).unwrap();

    // Use response.bytes() to get the image data
    let store = app_handle.store("store.json").unwrap();
    let mut games_data: GamesJson = store
        .get("gamesData")
        .map(|v| serde_json::from_value(v).unwrap())
        .unwrap_or(GamesJson { games_data: vec![] });

    game.image_url = String::from(path.to_str().unwrap());

    // TODO: Make games with same ID throw error / overwrite
    games_data.games_data.push(game);

    // TODO: For some reason the store structure is like this
    //{
    //  "gamesData": {
    //     "games_data": [
    store.set("gamesData", serde_json::to_value(games_data).unwrap());
    dbg!(store.get("gamesData").unwrap());
}
