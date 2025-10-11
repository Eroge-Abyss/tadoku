use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri_plugin_http::reqwest;

// POST Request
const VNDB_URL: &str = "https://api.vndb.org/kana";

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
    pub async fn get_vn_info(key: &str) -> Result<Vec<VndbGame>, String> {
        info!("Fetching game info for key: {}", key);
        let error_message = String::from("Error happened while fetching game");

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
            .map_err(|_| {
                // Idk if using clone is correct but, it should be dropped from stack anyway
                // + function will return on error
                error_message.clone()
            })
            .map_err(|_| {
                error!("Error sending request to VNDB for key: {}", key);
                error_message.clone()
            })?;

        if response.status() != 200 {
            error!(
                "VNDB returned status code: {} for key: {}",
                response.status(),
                key
            );
            return Err(error_message);
        }

        let json: VndbResponse<VndbGame> = response.json().await.map_err(|_| {
            error!("Error parsing VNDB response for key: {}", key);
            error_message
        })?;

        debug!("Successfully fetched game info for key: {}", key);
        Ok(json.results)
    }

    pub async fn get_vn_characters(vn_id: &str) -> Result<Vec<VndbCharacter>, String> {
        info!("Fetching characters for vn_id: {}", vn_id);
        let error_message = String::from("Error happened while fetching game");

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
            .map_err(|_| {
                // Idk if using clone is correct but, it should be dropped from stack anyway
                // + function will return on only of of the errors
                error_message.clone()
            })
            .map_err(|_| {
                error!("Error sending request to VNDB for vn_id: {}", vn_id);
                error_message.clone()
            })?;

        if response.status() != 200 {
            error!(
                "VNDB returned status code: {} for vn_id: {}",
                response.status(),
                vn_id
            );
            return Err(error_message);
        }

        let json: VndbResponse<VndbCharacter> = response.json().await.map_err(|e| {
            error!(
                "Error parsing VNDB response for vn_id: {:?}, error: {:?}",
                vn_id, e
            );
            error_message
        })?;

        debug!("Successfully fetched characters for vn_id: {}", vn_id);
        Ok(json.results)
    }

    pub async fn get_vns_alt_title(ids: &Vec<String>) -> Result<Vec<VndbAltTitleGame>, String> {
        let error_message = String::from("Error happened while fetching game");

        let id_filters: Vec<serde_json::Value> =
            ids.iter().map(|id| json!(["id", "=", id])).collect();

        let mut filters = vec![json!("or")];
        filters.extend(id_filters);

        let request_data = json!({
            "filters": filters,
            "fields": "id, alttitle"
        });

        debug!("Fetching alt titles for IDs: {:?}", ids);
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/{}", VNDB_URL, "vn"))
            .json(&request_data)
            .send()
            .await
            .map_err(|_| {
                // Idk if using clone is correct but, it should be dropped from stack anyway
                // + function will return on error
                error_message.clone()
            })
            .map_err(|_| {
                error!("Error sending request to VNDB for ids: {:?}", ids);
                error_message.clone()
            })?;

        if response.status() != 200 {
            return Err(error_message);
        }

        let json: VndbResponse<VndbAltTitleGame> = response.json().await.map_err(|_| {
            error!("Error parsing VNDB response for ids: {:?}", ids);
            error_message
        })?;

        debug!("Successfully fetched alt titles for IDs: {:?}", ids);
        Ok(json.results)
    }
}
