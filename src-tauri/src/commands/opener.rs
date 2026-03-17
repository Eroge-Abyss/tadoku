use crate::{
    AppState, GameState,
    commands::cmd_result::CmdResult,
    prelude::Fetchable,
    services::{discord::DiscordGameDetails, playtime, stores::games::GamesStore},
    util,
};
use anyhow::Context;
use log::{debug, error, info, warn};
use serde::Serialize;
use serde_json::json;
use std::{
    fs::{self},
    path::PathBuf,
    sync::Mutex,
    time::Duration,
};
use sysinfo::{Pid, System};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_shell::ShellExt;

#[derive(Serialize)]
pub struct ActiveWindow {
    title: String,
    exe_path: String,
    icon: Option<String>,
}

fn handle_open_lnk(exe_path: &mut PathBuf, args: &mut String) -> anyhow::Result<()> {
    debug!("Handling .lnk file: {:?}", exe_path);
    let lnk = lnk::ShellLink::open(&exe_path)
        .map_err(|e| anyhow::anyhow!("LNK error: {:?}", e))
        .context(format!("Error opening .lnk file {:?}", exe_path))?;

    let working_dir = lnk.working_dir().as_ref().context(format!(
        "Missing working directory in .lnk file: {:?}",
        exe_path
    ))?;
    let relative_path = lnk.relative_path().as_ref().context(format!(
        "Missing relative path in .lnk file: {:?}",
        exe_path
    ))?;
    *args = lnk
        .arguments()
        .as_ref()
        .unwrap_or(&String::new())
        .to_owned();
    *exe_path = fs::canonicalize(PathBuf::from(working_dir).join(relative_path))
        .context("Error resolving canonical path for .lnk file")?;

    debug!(
        "Successfully processed .lnk file. Resolved exe_path: {:?}, args: {:?}",
        exe_path, args
    );
    Ok(())
}

/// Opens a game and sets its PID in local state
#[tauri::command]
pub fn open_game(app_handle: AppHandle, game_id: String) -> CmdResult<()> {
    info!("Attempting to open game with ID: {}", game_id);

    let store = GamesStore::new(&app_handle).context("Error accessing store")?;

    if let Some(game) = store.get(&game_id) {
        info!(
            "Found game '{}' with exe path: {}",
            game.title, game.exe_file_path
        );
        let mut exe_path = PathBuf::from(&game.exe_file_path);
        let mut args = String::new();

        if exe_path.extension().unwrap_or_default() == "lnk" {
            debug!("Game {} uses .lnk file, processing...", game_id);
            handle_open_lnk(&mut exe_path, &mut args)
                .context("Error happened while handling .lnk file")?;
        }

        let current_dir = exe_path
            .parent()
            .context("Failed to get parent directory")?;

        let mut command = app_handle
            .shell()
            .command(&exe_path)
            .current_dir(current_dir);

        if !args.is_empty() {
            debug!("Starting game {} with arguments: {}", game_id, args);
            command = command.arg(args);
        } else {
            debug!("Starting game {} without arguments", game_id);
        }

        // Spawn is synchronous in Tauri 2.0 shell plugin, no block_on needed.
        command
            .spawn()
            .context("Error happened while running the game")?;

        info!("Successfully spawned game process for {}", game_id);

        tauri::async_runtime::spawn(async move {
            debug!("Starting game monitoring task for {}", game_id);
            let binding = app_handle.state::<Mutex<AppState>>();

            let pid;
            let mut counter: u8 = 0;

            loop {
                counter += 1;
                debug!(
                    "Looking for game process (attempt {}): {}",
                    counter, game.process_file_path
                );
                match util::get_pid_from_process_path(&game.process_file_path) {
                    Some(found_pid) => {
                        info!(
                            "Found game process for {} with PID: {}",
                            game_id,
                            found_pid.as_u32()
                        );
                        pid = found_pid;
                        break;
                    }
                    None => {
                        if counter > 60 {
                            error!(
                                "Timeout: Couldn't find game process for {} after 60 attempts",
                                game_id
                            );
                            return;
                        }
                        debug!(
                            "Game process not found yet for {}, retrying in 1 second...",
                            game_id
                        );
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                }
            }

            let mut state = match binding.lock() {
                Ok(s) => s,
                Err(e) => {
                    error!("Error acquiring mutex lock for game {}: {:?}", game_id, e);
                    return;
                }
            };

            state.game = Some(GameState {
                pid: pid.as_u32(),
                id: game_id.clone(),
                ..Default::default()
            });

            info!(
                "Set active game state for {} with PID {}",
                game_id,
                pid.as_u32()
            );

            let disable_presence_on_nsfw = state.settings.disable_presence_on_nsfw;
            let current_game_title = match &game.alt_title {
                Fetchable::Available(alt) if state.settings.use_jp_for_title_time => alt.clone(),
                _ => game.title.clone(),
            };

            if let Some(pres) = &mut state.presence {
                debug!("Setting Discord presence for game: {}", current_game_title);
                if let Err(e) = pres.set_presence(DiscordGameDetails::new(
                    &game_id,
                    &current_game_title,
                    &game.image_url,
                    game.is_nsfw && disable_presence_on_nsfw,
                )) {
                    error!("Error setting Discord presence for {}: {:?}", game_id, e);
                } else {
                    info!("Discord presence set for game: {}", current_game_title);
                }
            }

            debug!("Starting playtime tracking for {}", game_id);
            playtime::ClassicPlaytime::spawn(&app_handle);

            if let Err(e) = store.set_first_played(&game_id) {
                error!("Error setting first played for {}: {:?}", game_id, e);
            }

            if let Err(e) =
                app_handle.emit("current_game", json!({"id": game_id, "status": "playing"}))
            {
                error!("Error emitting current_game event for {}: {:?}", game_id, e);
            }

            info!("Game {} is now running and being tracked", game_id);
        });
    } else {
        return Err(anyhow::anyhow!("Game not found in store: {}", game_id).into());
    }

    Ok(())
}

/// Gets a list of open windows
#[tauri::command]
pub fn get_active_windows() -> CmdResult<Vec<ActiveWindow>> {
    debug!("Getting active windows list");

    #[cfg(windows)]
    {
        let open_windows = x_win::get_open_windows()
            .map_err(|_| anyhow::anyhow!("Error occurred while getting open windows"))?;

        debug!("Found {} active windows", open_windows.len());

        let active_windows: Vec<ActiveWindow> = open_windows
            .iter()
            .map(|window| {
                debug!("Processing window: {} ({})", window.title, window.info.path);
                ActiveWindow {
                    icon: x_win::get_window_icon(window).map(|i| i.data).ok(),
                    title: window.title.clone(),
                    exe_path: window.info.path.clone(),
                }
            })
            .collect();

        info!(
            "Successfully retrieved {} active windows",
            active_windows.len()
        );
        Ok(active_windows)
    }

    #[cfg(not(windows))]
    {
        debug!("get_active_windows called on non-Windows platform, returning empty list");
        Ok(Vec::new())
    }
}

/// Closes a game and clears local state
#[tauri::command]
pub fn close_game(app_handle: AppHandle) -> CmdResult<()> {
    info!("Attempting to close current game");

    let binding = app_handle.state::<Mutex<AppState>>();
    let state = binding
        .lock()
        .map_err(|_| anyhow::anyhow!("Failed to lock state"))?;

    let mut system = System::new_all();
    system.refresh_all();

    if let Some(game) = &state.game {
        info!("Closing game {} with PID {}", game.id, game.pid);
        let pid = Pid::from_u32(game.pid);
        if let Some(process) = system.process(pid) {
            debug!("Found process with PID {}, sending kill signal", game.pid);
            // Means the kill signal is successfully sent, doesn't mean the app has closed
            if process.kill() {
                info!("Kill signal sent successfully to PID {}", game.pid);
                process.wait();
                debug!("Process {} has been terminated", game.pid);
            } else {
                warn!("Failed to send kill signal to PID {}", game.pid);
            }
        } else {
            warn!("Process with PID {} not found in system", game.pid);
        }
    }

    Ok(())
}
