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

            let res: anyhow::Result<()> = async {
                let mut interval = tokio::time::interval(Duration::from_secs(1));
                interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
                interval.tick().await; // skip first immediate tick

                loop {
                    interval.tick().await;

                    let (pid, process_path, playtime_mode) = {
                        let state = app_handle.state::<ManagedState>();
                        let mut state = state.lock()?;
                        let game_state = state
                            .game
                            .as_mut()
                            .ok_or(anyhow!("Couldn't find the game"))?;
                        (
                            game_state.pid,
                            game_state.process_file_path.clone(),
                            state.settings.playtime_mode,
                        )
                    };

                    system.refresh_processes_specifics(
                        ProcessesToUpdate::Some(&[Pid::from_u32(pid)]),
                        true,
                        ProcessRefreshKind::everything(),
                    );

                    let process_exists = system.process(Pid::from_u32(pid)).is_some();

                    if !process_exists {
                        // Check if a child / preloader process is running under the configured path
                        let new_pid =
                            crate::services::system::SystemService::get_pid_from_process_path(
                                &process_path,
                            );
                        if let Some(new_pid) = new_pid.filter(|p| p.as_u32() != pid) {
                            debug!(
                                "Classic monitor: PID changed from {} to {}",
                                pid,
                                new_pid.as_u32()
                            );
                            if let Ok(mut state) = app_handle.state::<ManagedState>().lock() {
                                if let Some(ref mut game_state) = state.game {
                                    game_state.pid = new_pid.as_u32();
                                }
                            }
                            continue;
                        }

                        info!("Game process not found, stopping playtime tracking");
                        playtime_service.end_session();
                        break;
                    }

                    if !matches!(playtime_mode, PlaytimeMode::Classic) {
                        debug!("Playtime mode is not Classic, skipping tracking logic");
                        continue;
                    }

                    #[cfg(windows)]
                    {
                        if let Ok(active_window) = x_win::get_active_window() {
                            if active_window.id != pid {
                                playtime_service.pause_time();
                                continue;
                            }
                        } else {
                            debug!("Failed to get active window");
                            continue;
                        }
                    }

                    playtime_service.record_time(1);
                }
                Ok(())
            }
            .await;

            sync_handler.abort();
            res
        });
    }

    fn sync_handler(playtime_service: Arc<PlaytimeService>) -> JoinHandle<()> {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
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
