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
    id: String,
    name: String,
    image: CharacterImage,
    original: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct CharacterImage {
    url: String,
}

pub struct Vndb {}

impl Vndb {
    pub async fn get_vn_info(key: &str) -> Result<Vec<VndbGame>, String> {
        let error_message = String::from("Error happened while fetching game");

        let request_data = json!({
            "filters": ["search", "=", key],
            "fields": "id, title, image.url, image.sexual, description",
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
                // + function will return on only of of the errors
                error_message.clone()
            })?;

        if response.status() != 200 {
            return Err(error_message);
        }

        let json: VndbResponse<VndbGame> = response.json().await.map_err(|_| error_message)?;

        Ok(json.results)
    }

    pub async fn get_vn_characters(vn_id: &str) -> Result<Vec<VndbCharacter>, String> {
        let error_message = String::from("Error happened while fetching game");

        let request_data = json!({
            "filters": ["vn", "=", ["id", "=", vn_id]],
            "fields": "name, image.url, original"
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
            })?;

        if response.status() != 200 {
            return Err(error_message);
        }

        let json: VndbResponse<VndbCharacter> = response.json().await.map_err(|e| {
            dbg!(e);
            error_message
        })?;

        Ok(json.results)
    }
}
