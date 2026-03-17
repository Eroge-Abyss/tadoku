use anyhow::{Context, Result};
use log::{debug, info};
use serde::Deserialize;
use tauri_plugin_http::reqwest;

#[derive(Deserialize, Debug)]
pub struct DeckDetailResponse {
    pub data: Option<DeckDetailData>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeckDetailData {
    pub main_deck: Option<DeckDto>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeckDto {
    pub character_count: u64,
}

pub struct Jiten;

impl Jiten {
    pub async fn fetch_jiten_char_count(base_url: &str, game_id: &str) -> Result<Option<u64>> {
        info!("Fetching Jiten character count for game: {}", game_id);

        let base_url = base_url.trim_end_matches('/');
        let client = reqwest::Client::new();

        // Step 1: Look up deck IDs by VNDB link type (2 = Vndb)
        let lookup_url = format!("{}/api/media-deck/by-link-id/2/{}", base_url, game_id);
        debug!("Jiten lookup URL: {}", lookup_url);

        let response = client.get(&lookup_url).send().await.with_context(|| {
            format!("Failed sending Jiten lookup request for game: {}", game_id)
        })?;

        if !response.status().is_success() {
            info!(
                "Jiten API returned status {} for game {}",
                response.status(),
                game_id
            );
            return Ok(None);
        }

        let deck_ids: Vec<i32> = response.json().await.with_context(|| {
            format!(
                "Failed parsing Jiten deck IDs response for game: {}",
                game_id
            )
        })?;

        if deck_ids.is_empty() {
            info!("No Jiten deck found for game {}", game_id);
            return Ok(None);
        }

        let deck_id = deck_ids[0];
        info!("Found Jiten deck ID {} for game {}", deck_id, game_id);

        // Step 2: Fetch deck detail to get character count
        let detail_url = format!("{}/api/media-deck/{}/detail", base_url, deck_id);
        debug!("Jiten detail URL: {}", detail_url);

        let detail_response = client.get(&detail_url).send().await.with_context(|| {
            format!("Failed sending Jiten detail request for deck: {}", deck_id)
        })?;

        if !detail_response.status().is_success() {
            anyhow::bail!(
                "Jiten detail API returned status {} for deck {}",
                detail_response.status(),
                deck_id
            );
        }

        let detail: DeckDetailResponse = detail_response.json().await.with_context(|| {
            format!("Failed parsing Jiten detail response for deck: {}", deck_id)
        })?;

        if let Some(data) = detail.data {
            if let Some(main_deck) = data.main_deck {
                info!(
                    "Jiten character count for game {}: {}",
                    game_id, main_deck.character_count
                );
                return Ok(Some(main_deck.character_count));
            }
        }

        info!(
            "No character count data in Jiten response for game {}",
            game_id
        );
        Ok(None)
    }
}
