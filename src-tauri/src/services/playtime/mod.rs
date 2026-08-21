mod exstatic;
mod monitor;

use crate::services::{state::ManagedState, stores::games::GamesStore};
pub use exstatic::ExStaticPlaytime;
use log::{debug, error, warn};
pub use monitor::ProcessMonitor;
use tauri::{AppHandle, Emitter, Manager};

pub struct PlaytimeService {
    app_handle: AppHandle,
    store: GamesStore,
}

impl PlaytimeService {
    pub fn new(app_handle: AppHandle, store: GamesStore) -> Self {
        Self { app_handle, store }
    }

    pub fn record_time(&self, delta_seconds: u64) {
        let current_playtime = {
            let state = self.app_handle.state::<ManagedState>();
            let mut state = match state.lock() {
                Ok(s) => s,
                Err(e) => {
                    error!("Failed to acquire app state mutex lock: {}", e);
                    return;
                }
            };
            let game_state = match state.game.as_mut().ok_or("Couldn't find the game") {
                Ok(g) => g,
                Err(e) => {
                    warn!("No active game found: {}", e);
                    return;
                }
            };
            game_state.current_playtime += delta_seconds;
            game_state.unflushed_seconds += delta_seconds;
            game_state.current_playtime
        };

        if let Err(e) = self.app_handle.emit(
            "playtime",
            serde_json::json!({
                "status": "playing",
                "time": current_playtime
            }),
        ) {
            error!("Error happened while emitting playtime: {}", e);
        }
    }

    pub fn record_chars_read(&self, chars_count: u64) {
        let game_id = {
            let state = self.app_handle.state::<ManagedState>();
            let mut state = match state.lock() {
                Ok(s) => s,
                Err(e) => {
                    error!("Failed to acquire app state mutex lock: {}", e);
                    return;
                }
            };
            let game_state = match state.game.as_mut().ok_or("Couldn't find the game") {
                Ok(g) => g,
                Err(e) => {
                    warn!("No active game found: {}", e);
                    return;
                }
            };
            game_state.chars_read = chars_count;
            game_state.id.clone()
        };

        debug!(
            "Updating chars_read for game {} to {}",
            game_id, chars_count
        );
        if let Err(e) = self.app_handle.emit("chars_read_updated", chars_count) {
            error!("Error emitting chars_read_updated event: {}", e);
        }
    }

    #[allow(dead_code)]
    pub fn pause_time(&self) {
        let current_playtime = {
            let state = self.app_handle.state::<ManagedState>();
            let mut state = match state.lock() {
                Ok(s) => s,
                Err(e) => {
                    error!("Failed to acquire app state mutex lock: {}", e);
                    return;
                }
            };
            let game_state = match state.game.as_mut().ok_or("Couldn't find the game") {
                Ok(g) => g,
                Err(e) => {
                    warn!("No active game found: {}", e);
                    return;
                }
            };
            game_state.current_playtime
        };

        if let Err(e) = self.app_handle.emit(
            "playtime",
            serde_json::json!({
                "status": "paused",
                "time": current_playtime
            }),
        ) {
            error!("Error happened while emitting playtime: {}", e);
        }
    }

    pub fn flush(&self) -> crate::prelude::Result<()> {
        let (unflushed_seconds, chars_read, game_id) = {
            let state = self.app_handle.state::<ManagedState>();
            let mut state = state.lock()?;
            let game_state = match state.game.as_mut() {
                Some(g) => g,
                None => return Ok(()),
            };
            let res = (
                game_state.unflushed_seconds,
                game_state.chars_read,
                game_state.id.clone(),
            );
            game_state.unflushed_seconds = 0;
            res
        };

        if unflushed_seconds > 0 || chars_read > 0 {
            self.store
                .sync_game_session(&game_id, unflushed_seconds, Some(chars_read), false)?;

            if let Err(e) = self.app_handle.emit("stats_synced", ()) {
                error!("Error emitting stats_synced event: {}", e);
            }
        }

        Ok(())
    }

    pub fn end_session(&self) {
        let (unflushed_seconds, chars_read, game_id) = {
            let state = self.app_handle.state::<ManagedState>();
            let mut state = match state.lock() {
                Ok(s) => s,
                Err(e) => {
                    error!("Failed to acquire app state mutex lock: {}", e);
                    return;
                }
            };
            let game_state = match state.game.take() {
                Some(g) => (g.unflushed_seconds, g.chars_read, g.id),
                None => return,
            };

            if let Some(pres) = &mut state.presence {
                if let Err(e) = pres.reset_presence() {
                    error!("Error happened while clearing presence: {}", e);
                }
            }

            game_state
        };

        if let Err(e) =
            self.store
                .sync_game_session(&game_id, unflushed_seconds, Some(chars_read), true)
        {
            error!("Error saving session on exit: {}", e);
        } else if let Err(e) = self.app_handle.emit("stats_synced", ()) {
            error!("Error emitting stats_synced event: {}", e);
        }

        if let Err(e) = self
            .app_handle
            .emit("current_game", serde_json::json!(null))
        {
            error!("Error happened while emitting current_game event: {}", e);
        }
    }
}
