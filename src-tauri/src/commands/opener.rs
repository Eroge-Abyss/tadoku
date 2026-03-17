use crate::{
    commands::cmd_result::CmdResult,
    services::{game_manager::GameManager, system::SystemService},
};
use anyhow::Context;
use serde::Serialize;
use tauri::AppHandle;

#[derive(Serialize)]
pub struct ActiveWindow {
    title: String,
    exe_path: String,
    icon: Option<String>,
}

/// Opens a game and sets its PID in local state
// commands/opener.rs
#[tauri::command]
pub fn open_game(app_handle: AppHandle, game_id: String) -> CmdResult<()> {
    GameManager::new(&app_handle)
        .open(game_id)
        .context("Failed to open game")?;
    Ok(())
}

/// Gets a list of open windows
#[tauri::command]
pub fn get_active_windows() -> CmdResult<Vec<ActiveWindow>> {
    Ok(SystemService::get_active_windows()?)
}

/// Closes a game and clears local state
#[tauri::command]
pub fn close_game(app_handle: AppHandle) -> CmdResult<()> {
    GameManager::new(&app_handle)
        .close()
        .context("Failed to open game")?;
    Ok(())
}
