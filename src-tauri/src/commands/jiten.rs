use crate::services::jiten::Jiten;
use crate::{commands::cmd_result::CmdResult, services::state::ManagedState};
use anyhow::Context;
use tauri::{AppHandle, Manager};

/// Fetches the total character count for a VN from the Jiten API using its VNDB ID.
/// Returns None if the VN is not found in Jiten's database.
#[tauri::command]
pub async fn fetch_jiten_char_count(
    app_handle: AppHandle,
    game_id: String,
) -> CmdResult<Option<u64>> {
    let base_url = app_handle
        .state::<ManagedState>()
        .lock()?
        .settings
        .jiten_base_url
        .clone();

    let count = Jiten::fetch_jiten_char_count(&base_url, &game_id)
        .await
        .context(format!(
            "Error happened while fetching Jiten character count for game: {}",
            game_id
        ))?;

    Ok(count)
}
