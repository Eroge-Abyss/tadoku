use crate::prelude::Result;
use crate::services::state::{AppState, ManagedState};
use crate::services::stores::settings::SettingsStore;
use anyhow::Context;
use log::{debug, info};
use tauri::{AppHandle, Manager};

/// Initializes app state
pub fn initialize(app_handle: &AppHandle) -> Result<()> {
    info!("Initializing application state");

    let settings_store =
        SettingsStore::new(app_handle).context("Failed to create settings store")?;
    let settings = settings_store.load().context("Failed to load settings")?;

    debug!(
        "App config - disable_presence_on_nsfw: {}",
        settings.disable_presence_on_nsfw
    );
    debug!("App config - playtime_mode loaded successfully");
    debug!(
        "App config - use_jp_for_title_time: {}",
        settings.use_jp_for_title_time
    );

    let state = AppState {
        presence: None,
        settings,
        ..Default::default()
    };

    app_handle.manage(ManagedState::new(state));
    info!("Application state initialized successfully");

    Ok(())
}
