use serde::{Deserialize, Serialize};
use tauri_plugin_http::reqwest;

// POST Request
const VNDB_URL: &str = "https://api.vndb.org/kana/vn";

#[derive(Serialize)]
struct VndbRequest<'a> {
    filters: Vec<&'a str>,
    fields: &'a str,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct VndbResponse {
    more: bool,
    results: Vec<VndbGame>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VndbGame {
    id: String,
    title: String,
    image: Image,
    // String, possibly null, may contain formatting codes.
    description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Image {
    url: String,
}

#[tauri::command]
// Do I even need to do the deserialize / serialize thing or return the json as-is?
pub async fn fetch_vn_info(key: String) -> Result<Vec<VndbGame>, String> {
    let error_message = String::from("Error happened while fetching game");

    let request_data = VndbRequest {
        filters: vec!["search", "=", key.as_str()],
        fields: "id, title, image.url, description",
    };

    let client = reqwest::Client::new();
    let response = client
        .post(VNDB_URL)
        .json(&request_data)
        .send()
        .await
        .map_err(|e| {
            dbg!(&e);
            // Idk if using clone is correct but, it should be dropped from stack anyway
            // + function will return on only of of the errors
            error_message.clone()
        })?;

    if response.status() != 200 {
        return Err(error_message);
    }

    let json: VndbResponse = response.json().await.map_err(|e| {
        dbg!(&e);
        error_message
    })?;

    Ok(json.results)
}
