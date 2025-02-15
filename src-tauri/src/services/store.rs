use std::{collections::HashMap, error::Error, fs, path::PathBuf, sync::Arc};

use crate::util;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Wry};
use tauri_plugin_store::StoreExt;

type Store = Arc<tauri_plugin_store::Store<Wry>>;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub title: String,
    pub description: String,
    /// Is a local file path when loading games only, otherwise it's VNDB image URL
    pub image_url: String,
    pub exe_file_path: String,
    pub process_file_path: String,
    /// Play time in seconds
    pub playtime: u32,
    pub is_pinned: bool,
    pub is_nsfw: bool,
    pub icon_url: Option<String>,
    pub categories: Categories,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            image_url: String::new(),
            exe_file_path: String::new(),
            process_file_path: String::new(),
            playtime: 0,
            is_pinned: false,
            is_nsfw: false,
            icon_url: None,
            categories: Vec::new(),
        }
    }
}

pub type Games = HashMap<String, Game>;

pub struct GamesStore {
    store: Store,
    base_app_path: PathBuf,
}

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

    fn get_store(&self) -> serde_json::Value {
        self.store
            .get("gamesData")
            .unwrap_or_else(|| serde_json::json!({}))
    }

    /// Gets all games in store (image_urls are Paths)
    pub fn get_all(&self) -> Result<Games> {
        let mut games: Games = serde_json::from_value(self.get_store())?;

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
        let mut games: Games = serde_json::from_value(self.get_store())?;

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
        let mut games: Games = serde_json::from_value(self.get_store())?;
        games.insert(game_id, game_data);

        let games_data = serde_json::to_value(games)?;

        self.store.set("gamesData", games_data);

        Ok(())
    }

    /// Gets a game by id
    pub fn get(&self, game_id: &str) -> Option<Game> {
        let games_data = self.get_store();
        let game = games_data.get(game_id)?;

        serde_json::from_value::<Game>(game.clone()).ok()
    }

    /// Toggles a game's pinned state
    pub fn toggle_pin(&self, game_id: &str) -> Result<()> {
        let mut games_data = self.get_store();
        let game = games_data.get(game_id).ok_or("Couldn't find game")?;
        let mut game = serde_json::from_value::<Game>(game.clone())?;

        game.is_pinned = !game.is_pinned;
        games_data[game_id] = serde_json::to_value(game)?;

        self.store.set("gamesData", games_data);
        Ok(())
    }

    pub fn update_playtime(&self, game_id: &str, playtime: u64) -> Result<()> {
        let mut games_data = self.get_store();
        let game = games_data.get_mut(&game_id).ok_or("Couldn't find game")?;
        let old_playtime = game["playtime"].as_u64().ok_or("Can't convert to u64")?;

        game["playtime"] = serde_json::json!(old_playtime + playtime);
        self.store.set("gamesData", games_data);
        // Note
        // store.set() saves into an internal AppState
        // and the store is automatically saved to disk using auto_save (defined in default builder as 100ms)
        // why do I have to use it here even if it's every 60s? maybe a scope thing
        self.store.save()?;

        Ok(())
    }

    pub fn edit_exe(&self, game_id: &str, exe_path: &str) -> Result<()> {
        let mut games_data = self.get_store();
        let game = games_data.get_mut(&game_id).ok_or("Couldn't find game")?;

        game["exe_file_path"] = serde_json::json!(exe_path);
        self.store.set("gamesData", games_data);

        Ok(())
    }

    pub fn edit_process(&self, game_id: &str, process_path: &str) -> Result<()> {
        let mut games_data = self.get_store();
        let game = games_data.get_mut(&game_id).ok_or("Couldn't find game")?;

        game["process_file_path"] = serde_json::json!(process_path);
        self.store.set("gamesData", games_data);

        Ok(())
    }

    pub fn set_categories(&self, game_id: &str, categories: Categories) -> Result<()> {
        let mut games_data = self.get_store();
        let game = games_data.get_mut(&game_id).ok_or("Couldn't find game")?;

        game["categories"] = serde_json::json!(categories);
        self.store.set("gamesData", games_data);

        Ok(())
    }
}

pub struct CategoriesStore {
    store: Store,
}

pub type Categories = Vec<String>;

impl CategoriesStore {
    /// Creates store or uses existing one
    pub fn new(app_handle: &AppHandle) -> Result<Self> {
        let store = app_handle.store("store.json")?;

        Ok(Self { store })
    }

    fn get_store(&self) -> serde_json::Value {
        self.store
            .get("categories")
            .unwrap_or_else(|| serde_json::json!([]))
    }

    /// Gets all categories
    pub fn get_all(&self) -> Result<Categories> {
        Ok(serde_json::from_value(self.get_store())?)
    }

    /// Sets categories array to the provided value
    pub fn set(&self, categories: Categories) -> Result<()> {
        self.store.set("categories", categories);

        Ok(())
    }
}
