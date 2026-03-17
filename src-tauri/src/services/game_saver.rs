use crate::{
    prelude::{Fetchable, Result},
    services::{
        jiten,
        state::ManagedState,
        stores::games::{Character, Game, GamesStore},
        vndb::Vndb,
    },
    util,
};
use anyhow::Context;
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
#[cfg(windows)]
use windows_icons;

#[derive(Debug, Serialize, Deserialize)]
pub struct Options {
    pub include_characters: bool,
}

pub struct GameSaver<'a> {
    app_handle: &'a AppHandle,
}

impl<'a> GameSaver<'a> {
    pub fn new(app_handle: &'a AppHandle) -> Self {
        Self { app_handle }
    }

    pub async fn save(&self, game_id: String, mut game: Game, options: Options) -> Result<()> {
        let jiten_base_url = {
            let managed = self.app_handle.state::<ManagedState>();
            let lock = managed.lock()?;
            lock.settings.jiten_base_url.clone()
        };

        game = self.prepare_image(&game_id, game).await?;
        game = self.prepare_icon(&game_id, game)?;
        let (characters, jiten) = tokio::join!(
            self.fetch_characters(&game_id),
            self.fetch_jiten(&game_id, &jiten_base_url)
        );

        game.jiten_char_count = jiten?;

        if options.include_characters {
            game.characters = Some(characters?);
        } else {
            debug!("Skipping character fetching for game {}", game_id);
            game.characters = None;
        }

        self.persist(&game_id, game)?;

        Ok(())
    }

    async fn prepare_image(&self, game_id: &str, mut game: Game) -> Result<Game> {
        let _path: Option<String> = if game.image_url.is_empty() {
            debug!(
                "No image URL provided for game {}, skipping image save",
                game_id
            );
            None
        } else {
            // For local paths generate a unique dest name; for remote URLs derive from the URL.
            let dest_name = util::is_local_path(&game.image_url).then(|| {
                let filename =
                    util::extract_image(&game.image_url).unwrap_or_else(|_| "cover.jpg".to_owned());
                format!("{}_{}", game_id, filename)
            });
            let source = game.image_url.clone();
            // Update stored image_url to the final filename before saving.
            if let Some(ref name) = dest_name {
                game.image_url = name.clone();
            }
            let path = util::save_image(self.app_handle, &source, dest_name.as_deref())
                .await
                .context("Error happened while saving image")?;
            debug!("Successfully saved game image for {}", game_id);
            Some(path)
        };

        Ok(game)
    }

    fn prepare_icon(&self, game_id: &str, mut game: Game) -> Result<Game> {
        #[cfg(windows)]
        if let Some(ref saved_path) = _path {
            debug!("Extracting and saving icon for game {}", game_id);
            let icon = windows_icons::get_icon_by_path(&game.exe_file_path);
            let icon_path = format!("{}.icon.png", saved_path);
            icon.save(&icon_path)
                .context("Error happened while saving image")?;

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

        Ok(game)
    }

    pub async fn fetch_characters(&self, game_id: &str) -> Result<Vec<Character>> {
        info!("Fetching characters for game {}", game_id);
        let chars = Vndb::get_vn_characters(game_id)
            .await
            .context(format!("Error fetching characters for game {}", game_id))?;
        debug!("Found {} characters for game {}", chars.len(), game_id);

        let mut new_chars: Vec<Character> = Vec::new();

        for char in chars {
            debug!("Processing character: {} (ID: {})", char.name, char.id);
            let path = match char.image {
                Some(p) => {
                    debug!("Saving character image for {} ({})", char.name, p.url);
                    Some(
                        util::save_image(self.app_handle, &p.url, None)
                            .await
                            .context("Error happened while saving image")?,
                    )
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

        Ok(new_chars)
    }

    async fn fetch_jiten(&self, game_id: &str, base_url: &str) -> Result<Fetchable<u64>> {
        let res = match jiten::Jiten::fetch_jiten_char_count(base_url, game_id).await {
            Ok(Some(count)) => {
                info!(
                    "Successfully fetched Jiten character count ({}) for game {}",
                    count, game_id
                );
                Fetchable::Available(count)
            }
            Ok(None) => {
                info!("No Jiten character count found for game {}", game_id);
                Fetchable::NotFound
            }
            Err(e) => {
                warn!("Jiten fetch failed for {}: {}", game_id, e);
                Fetchable::NotFetched
            }
        };

        Ok(res)
    }

    fn persist(&self, game_id: &str, game: Game) -> Result<()> {
        let store =
            GamesStore::new(self.app_handle).context("Error happened while accessing store")?;

        store
            .save(game_id.to_string(), game)
            .context("Error happened while saving game")?;

        Ok(())
    }
}
