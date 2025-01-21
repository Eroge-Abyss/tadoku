use serde::Serialize;
use serde_json::Value;
use tauri_plugin_http::reqwest;

// POST Request
const VNDB_URL: &str = "https://api.vndb.org/kana/vn";

#[derive(Serialize)]
struct VndbRequest<'a> {
    filters: Vec<&'a str>,
    fields: &'a str,
}

#[tauri::command]
pub async fn fetch_vn_info(key: String) {
    let request_data = VndbRequest {
        filters: vec!["search", "=", key.as_str()],
        fields: "title, image.url",
    };

    // TODO: Add error handling
    let client = reqwest::Client::new();
    let result = client
        .post(VNDB_URL)
        .json(&request_data)
        .send()
        .await
        .unwrap();

    dbg!(&result);
    // TODO: Make strong type for this
    let json: Value = result.json().await.unwrap();
    dbg!(&json);
}
