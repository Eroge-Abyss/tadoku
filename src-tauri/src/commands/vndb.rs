use crate::services::vndb::{Vndb, VndbGame};
use log::{debug, error, info};

#[tauri::command]
// Do I even need to do the deserialize / serialize thing or return the json as-is?
pub async fn fetch_vn_info(key: String) -> Result<Vec<VndbGame>, String> {
    info!("Fetching VN info for key: {}", key);
    debug!("Starting VNDB API request for: {}", key);

    match Vndb::get_vn_info(&key).await {
        Ok(games) => {
            info!(
                "Successfully fetched {} VN(s) for key: {}",
                games.len(),
                key
            );
            debug!("VN data retrieved successfully from VNDB API");
            Ok(games)
        }
        Err(e) => {
            error!("Failed to fetch VN info for key '{}': {}", key, e);
            Err(e)
        }
    }
}
