use crate::services::vndb::{Vndb, VndbGame};

#[tauri::command]
// Do I even need to do the deserialize / serialize thing or return the json as-is?
pub async fn fetch_vn_info(key: String) -> Result<Vec<VndbGame>, String> {
    Vndb::get_vn_info(&key).await
}
