use crate::services::playtime::PlaytimeService;
use crate::services::stores::games::GamesStore;
use crate::services::{state::ManagedState, stores::settings::PlaytimeMode};
use anyhow::anyhow;
use log::{debug, error, info};
use std::sync::Arc;
use std::time::Duration;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System};
use tauri::{AppHandle, Manager};
use tokio::task::JoinHandle;

pub struct ProcessMonitor;

impl ProcessMonitor {
    pub fn spawn(app_handle: &AppHandle) {
        info!("Spawning classic playtime tracking task");
        let app_handle = app_handle.clone();
        tauri::async_runtime::spawn(async move {
            debug!("Classic playtime task started");
            let store = GamesStore::new(&app_handle)?;
            let playtime_service = Arc::new(PlaytimeService::new(app_handle.clone(), store));
            let mut system = System::new_with_specifics(
                RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
            );

            let sync_handler = Self::sync_handler(Arc::clone(&playtime_service));

            loop {
                let (pid, playtime_mode) = {
                    let state = app_handle.state::<ManagedState>();
                    let mut state = state.lock()?;
                    let game_state = state
                        .game
                        .as_mut()
                        .ok_or(anyhow!("Couldn't find the game"))?;
                    (game_state.pid, state.settings.playtime_mode)
                };

                system.refresh_processes_specifics(
                    ProcessesToUpdate::Some(&[Pid::from_u32(pid)]),
                    true,
                    ProcessRefreshKind::everything(),
                );

                let process_playtime = system.process(Pid::from_u32(pid)).map(|p| p.run_time());

                match process_playtime {
                    Some(_) => {
                        debug!("Sleeping for 1 second");
                        tokio::time::sleep(Duration::from_secs(1)).await;

                        if !matches!(playtime_mode, PlaytimeMode::Classic) {
                            debug!("Playtime mode is not Classic, skipping iteration");
                            continue;
                        }

                        debug!("Playtime mode is Classic, proceeding with tracking");

                        #[cfg(windows)]
                        {
                            if let Ok(active_window) = x_win::get_active_window() {
                                if active_window.id != pid {
                                    debug!("Game is not active, pausing playtime");
                                    playtime_service.pause_time();
                                    continue;
                                }
                            } else {
                                debug!("Failed to get active window");
                                continue;
                            }
                        }

                        debug!("Game is active, incrementing playtime");

                        playtime_service.record_time(1);
                    }
                    None => {
                        info!("Game process not found, stopping playtime tracking");
                        sync_handler.abort();
                        playtime_service.end_session();
                        break;
                    }
                }
            }

            anyhow::Ok(())
        });
    }

    fn sync_handler(playtime_service: Arc<PlaytimeService>) -> JoinHandle<()> {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            // First tick completes immediately, skip
            interval.tick().await;

            loop {
                interval.tick().await;
                if let Err(e) = playtime_service.flush() {
                    error!("Error during periodic playtime flush: {}", e);
                }
                debug!("Flushed updated state to disk")
            }
        })
    }
}
