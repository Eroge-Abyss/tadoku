mod character;
mod game;
use crate::prelude::*;
use crate::util;
pub use character::Character;
use chrono::Local;
pub use game::Game;
use log::{debug, info};
use std::{collections::HashMap, fs, path::PathBuf, time};
use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreExt;

pub type Games = HashMap<String, Game>;

pub struct GamesStore {
    store: Store,
    base_app_path: PathBuf,
}

impl GamesStore {
    /// Creates store or uses existing one
    pub fn new(app_handle: &AppHandle) -> Result<Self> {
        debug!("Creating GamesStore");
        let store = app_handle.store("store.json")?;
        let base_app_path = app_handle.path().app_local_data_dir()?;

        Ok(Self {
            store,
            base_app_path,
        })
    }

    fn get_store_value(&self) -> serde_json::Value {
        debug!("Getting games from store");
        self.store
            .get("gamesData")
            .unwrap_or_else(|| serde_json::json!({}))
    }

    /// Gets all games in store (image_urls are Paths)
    pub fn get_all(&self) -> Result<Games> {
        debug!("Getting all games");
        let mut games: Games = serde_json::from_value(self.get_store_value())?;

        for game in games.values_mut() {
            game.image_url = util::construct_image_path(&self.base_app_path, &game.image_url)?
                .to_str()
                .ok_or("Error happened while constructing image path")?
                .to_string();
        }
        Ok(games)
    }

    /// Deletes a game from the store (also removes images from filesystem)
    pub fn delete(&self, game_id: &str) -> Result<()> {
        info!("Deleting game with id: {}", game_id);
        let mut games: Games = serde_json::from_value(self.get_store_value())?;

        if let Some(removed_game) = games.remove(game_id) {
            fs::remove_file(util::construct_image_path(
                &self.base_app_path,
                &removed_game.image_url,
            )?)
            .map_err(|err| err.to_string())?;

            if let Some(characters) = removed_game.characters {
                for character in characters {
                    if let Some(image_url) = character.image_url {
                        if fs::metadata(&image_url).is_ok() {
                            let _ = fs::remove_file(&image_url);
                        }
                    }
                }
            }

            self.store.set("gamesData", serde_json::to_value(games)?);
            self.store.save()?;
        }

        Ok(())
    }

    /// Resets a game's stats (playtime, today_playtime, last_played, first_played and last_play_date)
    pub fn reset_stats(&self, game_id: &str) -> Result<()> {
        info!("Resetting stats for game with id: {}", game_id);
        self.update_game(game_id, |game| {
            game.playtime = 0;
            game.today_playtime = 0;
            game.last_played = None;
            game.first_played = None;
            game.last_play_date = None;
            game.chars_read = 0;
        })
    }

    /// Saves a game to the store
    pub fn save(&self, game_id: String, game_data: Game) -> Result<()> {
        info!("Saving game with id: {}", game_id);
        let mut games: Games = serde_json::from_value(self.get_store_value())?;
        games.insert(game_id, game_data);

        self.store.set("gamesData", serde_json::to_value(games)?);
        self.store.save()?;

        Ok(())
    }

    /// Generic update method for a single game.
    /// Handles loading, updating, and saving in one transaction.
    pub fn update_game<F>(&self, game_id: &str, update_fn: F) -> Result<()>
    where
        F: FnOnce(&mut Game),
    {
        debug!("Updating game with id: {}", game_id);
        let mut games: Games = serde_json::from_value(self.get_store_value())?;

        if let Some(game) = games.get_mut(game_id) {
            update_fn(game);
            self.store.set("gamesData", serde_json::to_value(games)?);
            self.store.save()?;
            Ok(())
        } else {
            Err(format!("Game with id {} not found", game_id).into())
        }
    }

    /// Gets a game by id
    pub fn get(&self, game_id: &str) -> Option<Game> {
        debug!("Getting game with id: {}", game_id);
        let games_data = self.get_store_value();
        let game_val = games_data.get(game_id)?;

        serde_json::from_value::<Game>(game_val.clone()).ok()
    }

    /// Logic-heavy helpers that benefit from being in the store layer
    pub fn update_playtime(&self, game_id: &str, playtime: u64) -> Result<()> {
        self.update_game(game_id, |game| {
            game.playtime += playtime;
            let current_date = Local::now().format("%Y-%m-%d").to_string();

            if game.last_play_date.as_ref() != Some(&current_date) {
                game.today_playtime = playtime;
                game.last_play_date = Some(current_date);
            } else {
                game.today_playtime += playtime;
            }
        })
    }

    pub fn update_last_played(&self, game_id: &str) -> Result<()> {
        self.update_game(game_id, |game| {
            let start = time::SystemTime::now();
            let since_the_epoch = start
                .duration_since(time::UNIX_EPOCH)
                .expect("Time went backwards");
            game.last_played = Some(since_the_epoch.as_secs());
        })
    }

    pub fn set_first_played(&self, game_id: &str) -> Result<()> {
        self.update_game(game_id, |game| {
            if game.first_played.is_none() {
                let start = time::SystemTime::now();
                let since_the_epoch = start
                    .duration_since(time::UNIX_EPOCH)
                    .expect("Time went backwards");
                game.first_played = Some(since_the_epoch.as_secs());
            }
        })
    }

    pub fn update_chars_read(&self, game_id: &str, chars_read: u64) -> Result<()> {
        self.update_game(game_id, |game| {
            game.chars_read = chars_read;
        })
    }
}
