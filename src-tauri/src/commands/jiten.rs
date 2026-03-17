use crate::AppState;
use crate::commands::cmd_result::CmdResult;
use crate::services::jiten::Jiten;
use anyhow::Context;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

/// Fetches the total character count for a VN from the Jiten API using its VNDB ID.
/// Returns None if the VN is not found in Jiten's database.
#[tauri::command]
pub async fn fetch_jiten_char_count(
    app_handle: AppHandle,
    game_id: String,
) -> CmdResult<Option<u64>> {
    let base_url = {
        let state = app_handle.state::<Mutex<AppState>>();
        let lock = state
            .lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock state"))?;
        lock.settings.jiten_base_url.clone()
    };

    let count = Jiten::fetch_jiten_char_count(&base_url, &game_id)
        .await
        .context(format!(
            "Error happened while fetching Jiten character count for game: {}",
            game_id
        ))?;

    Ok(count)
}
