use crate::services::state::ManagedState;
use crate::services::discord::DiscordPresence;
use log::{debug, error, info, warn};
use tauri::{AppHandle, Manager};



pub fn initialize(app_handle: &AppHandle) -> tauri::Result<()> {
    info!("Initializing Discord presence");
    let app_handle_clone = app_handle.clone();

    tauri::async_runtime::spawn(async move {
        debug!("Starting Discord initialization background task");
        let app_state_mutex = app_handle_clone.state::<ManagedState>();

        let mut state = match app_state_mutex.lock() {
            Ok(s) => {
                debug!("Background task: Successfully acquired app state mutex lock");
                s
            }
            Err(e) => {
                error!(
                    "Background task: Error acquiring mutex lock for AppState: {}",
                    e
                );
                return Ok(());
            }
        };

        let mode = state.settings.discord_presence_mode;

        debug!("Background task: Retrieved Discord presence mode from settings");

        match DiscordPresence::new(mode) {
            Ok(presence) => {
                info!("Background task: Successfully initialized Discord presence");
                state.presence = Some(presence);
            }
            Err(e) => {
                warn!(
                    "Background task: Failed to initialize DiscordPresence: {}",
                    e
                );
                debug!("Background task: Discord presence will be disabled");
                state.presence = None;
            }
        }

        debug!("Background task: Discord initialization completed");
        std::result::Result::<(), String>::Ok(())
    });

    info!("Discord initialization task spawned");
    Ok(())
}