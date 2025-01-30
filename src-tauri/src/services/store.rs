use std::{collections::HashMap, error::Error, fs, path::PathBuf, sync::Arc};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Wry};
use tauri_plugin_store::{Store, StoreExt};

use crate::util;

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub title: String,
    pub description: String,
    /// Is a local file path when loading games only, otherwise it's VNDB image URL
    pub image_url: String,
    pub exe_file_path: String,
    /// Play time in seconds
    pub playtime: u32,
    pub is_pinned: bool,
}

pub type Games = HashMap<String, Game>;

pub struct GamesStore {
    store: Arc<Store<Wry>>,
    base_app_path: PathBuf,
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

impl GamesStore {
    /// Creates store or uses existing one
    pub fn new(app_handle: &AppHandle) -> Result<Self> {
        let store = app_handle.store("store.json")?;
        let base_app_path = app_handle.path().app_local_data_dir()?;

        Ok(Self {
            store,
            base_app_path,
        })
    }

    /// Gets all games in store (image_urls are Paths)
    pub fn get_all(&self) -> Result<Games> {
        let games_data = self
            .store
            .get("gamesData")
            .unwrap_or_else(|| serde_json::json!({}));

        let mut games: Games = serde_json::from_value(games_data)?;

        for game in games.values_mut() {
            game.image_url = util::construct_image_path(&self.base_app_path, &game.image_url)?
                .to_str()
                .ok_or("Error happened while constructing image path")?
                .to_string();
        }
        Ok(games)
    }

    /// Deletes a game from the store (also removes image from filesystem)
    pub fn delete(&self, game_id: &str) -> Result<()> {
        let games_data = self
            .store
            .get("gamesData")
            .unwrap_or_else(|| serde_json::json!({}));

        let mut games: Games = serde_json::from_value(games_data)?;

        if let Some(removed_game) = games.remove(game_id) {
            fs::remove_file(util::construct_image_path(
                &self.base_app_path,
                &removed_game.image_url,
            )?)
            .map_err(|err| err.to_string())?;
            let games_data = serde_json::to_value(games)?;
            self.store.set("gamesData", games_data);
        }

        Ok(())
    }

    /// Saves a game to the store
    pub fn save(&self, game_id: String, game_data: Game) -> Result<()> {
        let games_data = self
            .store
            .get("gamesData")
            .unwrap_or_else(|| serde_json::json!({}));

        let mut games: Games = serde_json::from_value(games_data)?;
        games.insert(game_id, game_data);

        let games_data = serde_json::to_value(games)?;

        self.store.set("gamesData", games_data);

        Ok(())
    }

    /// Gets a game by id
    pub fn get(&self, game_id: &str) -> Option<Game> {
        let games_data = self
            .store
            .get("gamesData")
            .unwrap_or_else(|| serde_json::json!({}));

        let game = games_data.get(game_id)?;

        serde_json::from_value::<Game>(game.clone()).ok()
    }

    /// Toggles a game's pinned state
    pub fn toggle_pin(&self, game_id: &str) -> Result<()> {
        let mut games_data = self
            .store
            .get("gamesData")
            .unwrap_or_else(|| serde_json::json!({}));

        let game = games_data.get(game_id).ok_or("Couldn't find game")?;
        let mut game = serde_json::from_value::<Game>(game.clone())?;

        game.is_pinned = !game.is_pinned;
        games_data[game_id] = serde_json::to_value(game)?;

        self.store.set("gamesData", games_data);
        Ok(())
    }

    pub fn update_playtime(&self, game_id: &str, playtime: u64) -> Result<()> {
        let mut games_data = self
            .store
            .get("gamesData")
            .unwrap_or_else(|| serde_json::json!({}));

        let game = games_data.get_mut(&game_id).ok_or("Couldn't find game")?;
        let old_playtime = game["playtime"].as_u64().ok_or("Can't convert to u64")?;

        game["playtime"] = serde_json::json!(old_playtime + playtime);
        self.store.set("gamesData", games_data);

        Ok(())
    }
}
