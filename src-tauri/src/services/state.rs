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
    pub process_file_path: String,
    pub current_playtime: u64,
    pub unflushed_seconds: u64,
    pub chars_read: u64,
}

impl GameState {
    pub fn matches_path(&self, incoming: &str) -> bool {
        if incoming.trim().is_empty() || self.process_file_path.trim().is_empty() {
            return false;
        }

        #[cfg(windows)]
        {
            self.process_file_path.eq_ignore_ascii_case(incoming)
        }

        #[cfg(not(windows))]
        {
            use crate::services::system::SystemService;
            let norm_incoming = SystemService::normalize_wine_path(incoming).to_lowercase();
            let norm_configured =
                SystemService::normalize_wine_path(&self.process_file_path).to_lowercase();
            norm_incoming == norm_configured
                || norm_configured.contains(&norm_incoming)
                || norm_incoming.contains(&norm_configured)
        }
    }
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
