use crate::{
    prelude::Result,
    services::{
        discord::DiscordPresence,
        stores::settings::{Settings, SettingsStore},
    },
};
use std::sync::{Mutex, MutexGuard};
use tauri::AppHandle;

#[derive(Default, Clone)]
pub struct GameState {
    pub id: String,
    pub pid: u32,
    pub current_playtime: u64,
}

#[derive(Default)]
pub struct AppState {
    pub game: Option<GameState>,
    pub presence: Option<DiscordPresence>,
    pub settings: Settings,
}

impl AppState {
    pub fn update_settings<F>(&mut self, app_handle: &AppHandle, update_fn: F) -> Result<()>
    where
        F: FnOnce(&mut Settings),
    {
        update_fn(&mut self.settings);

        let store = SettingsStore::new(app_handle)?;
        store.save(&self.settings)?;

        Ok(())
    }
}

pub struct ManagedState(Mutex<AppState>);

impl ManagedState {
    pub fn new(inner: AppState) -> Self {
        Self(Mutex::new(inner))
    }

    pub fn lock(&self) -> Result<MutexGuard<'_, AppState>> {
        self.0
            .lock()
            .map_err(|_| anyhow::anyhow!("State mutex poisoned"))
    }
}
