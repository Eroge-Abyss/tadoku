use log::{debug, error, info};
use serde::Deserialize;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use tauri_plugin_http::reqwest;

use crate::AppState;

#[derive(Deserialize, Debug)]
struct DeckDetailResponse {
    data: Option<DeckDetailData>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DeckDetailData {
    main_deck: Option<DeckDto>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DeckDto {
    character_count: u64,
}

/// Fetches the total character count for a VN from the Jiten API using its VNDB ID.
/// Returns None if the VN is not found in Jiten's database.
#[tauri::command]
pub async fn fetch_jiten_char_count(
    app_handle: AppHandle,
    game_id: String,
) -> Result<Option<u64>, String> {
    info!("Fetching Jiten character count for game: {}", game_id);

    let base_url = {
        let binding = app_handle.state::<Mutex<AppState>>();
        let state = binding.lock().map_err(|e| {
            error!("Error acquiring mutex lock: {}", e);
            "Error acquiring mutex lock".to_string()
        })?;
        state.settings.jiten_base_url.clone()
    };

    let base_url = base_url.trim_end_matches('/');

    // Step 1: Look up deck IDs by VNDB link type (2 = Vndb)
    let lookup_url = format!(
        "{}/api/media-deck/by-link-id/2/{}",
        base_url, game_id
    );
    debug!("Jiten lookup URL: {}", lookup_url);

    let client = reqwest::Client::new();

    let deck_ids: Vec<i32> = match client.get(&lookup_url).send().await {
        Ok(response) => {
            if !response.status().is_success() {
                info!(
                    "Jiten API returned status {} for game {}",
                    response.status(),
                    game_id
                );
                return Ok(None);
            }
            response.json().await.map_err(|e| {
                error!("Failed to parse Jiten deck IDs response: {}", e);
                e.to_string()
            })?
        }
        Err(e) => {
            error!("Failed to fetch Jiten deck IDs: {}", e);
            return Ok(None);
        }
    };

    if deck_ids.is_empty() {
        info!("No Jiten deck found for game {}", game_id);
        return Ok(None);
    }

    let deck_id = deck_ids[0];
    info!("Found Jiten deck ID {} for game {}", deck_id, game_id);

    // Step 2: Fetch deck detail to get character count
    let detail_url = format!("{}/api/media-deck/{}/detail", base_url, deck_id);
    debug!("Jiten detail URL: {}", detail_url);

    let detail: DeckDetailResponse = match client.get(&detail_url).send().await {
        Ok(response) => {
            if !response.status().is_success() {
                error!(
                    "Jiten detail API returned status {} for deck {}",
                    response.status(),
                    deck_id
                );
                return Ok(None);
            }
            response.json().await.map_err(|e| {
                error!("Failed to parse Jiten detail response: {}", e);
                e.to_string()
            })?
        }
        Err(e) => {
            error!("Failed to fetch Jiten deck detail: {}", e);
            return Ok(None);
        }
    };

    if let Some(data) = detail.data {
        if let Some(main_deck) = data.main_deck {
            info!(
                "Jiten character count for game {}: {}",
                game_id, main_deck.character_count
            );
            return Ok(Some(main_deck.character_count));
        }
    }

    info!("No character count data in Jiten response for game {}", game_id);
    Ok(None)
}

