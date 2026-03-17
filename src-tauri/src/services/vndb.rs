use crate::prelude::Result;
use anyhow::Context;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri_plugin_http::reqwest;

// POST Request
const VNDB_URL: &str = "https://api.vndb.org/kana";
pub const VNDB_MAX_PAGE_SIZE: usize = 50;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct VndbResponse<T> {
    more: bool,
    results: Vec<T>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VndbGame {
    id: String,
    title: String,
    alttitle: Option<String>,
    image: GameImage,
    // String, possibly null, may contain formatting codes.
    description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct GameImage {
    url: String,
    sexual: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VndbCharacter {
    pub id: String,
    pub name: String,
    pub image: Option<CharacterImage>,
    pub original: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CharacterImage {
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VndbAltTitleGame {
    pub id: String,
    pub alttitle: Option<String>,
}

pub struct Vndb;

impl Vndb {
    pub async fn get_vn_info(key: &str) -> Result<Vec<VndbGame>> {
        info!("Fetching game info for key: {}", key);

        let request_data = json!({
            "filters": ["search", "=", key],
            "fields": "id, title, alttitle, image.url, image.sexual, description",
            "sort": "searchrank",
        });

        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/{}", VNDB_URL, "vn"))
            .json(&request_data)
            .send()
            .await
            .with_context(|| format!("Failed sending VNDB request for key: {}", key))?;

        if !response.status().is_success() {
            error!(
                "VNDB returned status code: {} for key: {}",
                response.status(),
                key
            );
            anyhow::bail!(
                "VNDB returned status code {} for key {}",
                response.status(),
                key
            );
        }

        let json: VndbResponse<VndbGame> = response
            .json()
            .await
            .with_context(|| format!("Failed parsing VNDB response for key: {}", key))?;

        debug!("Successfully fetched game info for key: {}", key);
        Ok(json.results)
    }

    pub async fn get_vn_characters(vn_id: &str) -> Result<Vec<VndbCharacter>> {
        info!("Fetching characters for vn_id: {}", vn_id);

        let request_data = json!({
            "filters": ["vn", "=", ["id", "=", vn_id]],
            "fields": "name, image.url, original",
            "results": "100"
        });

        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/{}", VNDB_URL, "character"))
            .json(&request_data)
            .send()
            .await
            .with_context(|| {
                format!("Failed sending VNDB character request for vn_id: {}", vn_id)
            })?;

        if !response.status().is_success() {
            error!(
                "VNDB returned status code: {} for vn_id: {}",
                response.status(),
                vn_id
            );
            anyhow::bail!(
                "VNDB character request returned status code {} for vn_id {}",
                response.status(),
                vn_id
            );
        }

        let json: VndbResponse<VndbCharacter> = response.json().await.with_context(|| {
            format!(
                "Failed parsing VNDB character response for vn_id: {}",
                vn_id
            )
        })?;

        debug!("Successfully fetched characters for vn_id: {}", vn_id);
        Ok(json.results)
    }

    pub async fn get_vns_alt_title(ids: &[String]) -> Result<Vec<VndbAltTitleGame>> {
        let id_filters: Vec<serde_json::Value> =
            ids.iter().map(|id| json!(["id", "=", id])).collect();

        let mut filters = vec![json!("or")];
        filters.extend(id_filters);

        let request_data = json!({
            "filters": filters,
            "fields": "id, alttitle",
            "results": VNDB_MAX_PAGE_SIZE
        });

        debug!("Fetching alt titles for IDs: {:?}", ids);
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/{}", VNDB_URL, "vn"))
            .json(&request_data)
            .send()
            .await
            .with_context(|| format!("Failed sending VNDB alt-title request for ids: {:?}", ids))?;

        if !response.status().is_success() {
            anyhow::bail!(
                "VNDB alt-title request returned status code {} for ids {:?}",
                response.status(),
                ids
            );
        }

        let json: VndbResponse<VndbAltTitleGame> = response.json().await.with_context(|| {
            format!("Failed parsing VNDB alt-title response for ids: {:?}", ids)
        })?;

        debug!("Successfully fetched alt titles for IDs: {:?}", ids);

        Ok(json.results)
    }
}
