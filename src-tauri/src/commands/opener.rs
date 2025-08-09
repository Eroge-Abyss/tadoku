use crate::{
    services::{discord::DiscordGameDetails, playtime, stores::games::GamesStore},
    util, AppState, GameState,
};
use log::{debug, error, info, warn};
use serde::Serialize;
use serde_json::json;
use std::{
    fs::{self},
    path::PathBuf,
    sync::Mutex,
    thread,
    time::Duration,
};
use sysinfo::{Pid, System};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_shell::ShellExt;

#[derive(Serialize)]
pub struct ActiveWindow {
    title: String,
    exe_path: String,
    icon: String,
}

fn handle_open_lnk(exe_path: &mut PathBuf, args: &mut String) -> Result<(), String> {
    debug!("Handling .lnk file: {:?}", exe_path);
    let lnk = lnk::ShellLink::open(&exe_path).map_err(|e| {
        error!("Error opening .lnk file {:?}: {:?}", exe_path, e);
        "Error opening .lnk file"
    })?;

    let working_dir = lnk.working_dir().as_ref().ok_or_else(|| {
        error!("Missing working directory in .lnk file: {:?}", exe_path);
        "Missing working directory in .lnk file"
    })?;
    let relative_path = lnk.relative_path().as_ref().ok_or_else(|| {
        error!("Missing relative path in .lnk file: {:?}", exe_path);
        "Missing relative path in .lnk file"
    })?;
    *args = lnk
        .arguments()
        .as_ref()
        .unwrap_or(&String::new())
        .to_owned();
    *exe_path = fs::canonicalize(PathBuf::from(working_dir).join(relative_path)).map_err(|e| {
        error!("Error resolving canonical path for .lnk file: {}", e);
        format!("Error resolving canonical path: {}", e)
    })?;

    debug!(
        "Successfully processed .lnk file. Resolved exe_path: {:?}, args: {:?}",
        exe_path, args
    );
    Ok(())
}

/// Opens a game and sets its PID in local state
#[tauri::command]
pub fn open_game(app_handle: AppHandle, game_id: String) -> Result<(), String> {
    info!("Attempting to open game with ID: {}", game_id);

    let store = GamesStore::new(&app_handle).map_err(|e| {
        error!("Error accessing store for game {}: {}", game_id, e);
        format!("Error accessing store: {}", e)
    })?;

    if let Some(game) = store.get(&game_id) {
        info!(
            "Found game '{}' with exe path: {}",
            game.title, game.exe_file_path
        );
        let mut exe_path = PathBuf::from(&game.exe_file_path);
        let mut args = String::new();

        if exe_path.extension().unwrap_or_default() == "lnk" {
            debug!("Game {} uses .lnk file, processing...", game_id);
            handle_open_lnk(&mut exe_path, &mut args)?;
        }

        tauri::async_runtime::block_on(async {
            let mut command = app_handle
                .shell()
                .command(&exe_path)
                .current_dir(exe_path.parent().ok_or("Failed to get parent directory")?);

            if !args.is_empty() {
                debug!("Starting game {} with arguments: {}", game_id, args);
                command = command.arg(args);
            } else {
                debug!("Starting game {} without arguments", game_id);
            }

            let _ = command.spawn().map_err(|e| {
                error!("Error spawning game process for {}: {:?}", game_id, e);
                "Error happened while running the game".to_string()
            })?;

            info!("Successfully spawned game process for {}", game_id);

            Result::<(), String>::Ok(())
        })?;

        tauri::async_runtime::spawn(async move {
            debug!("Starting game monitoring task for {}", game_id);
            let binding = app_handle.state::<Mutex<AppState>>();
            let mut state = binding.lock().map_err(|e| {
                error!("Error acquiring mutex lock for game {}: {:?}", game_id, e);
                "Error acquiring mutex lock".to_string()
            })?;

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
                            return Err(String::from("Couldn't find game process"));
                        }
                        debug!(
                            "Game process not found yet for {}, retrying in 1 second...",
                            game_id
                        );
                        thread::sleep(Duration::from_secs(1))
                    }
                }
            }

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

            let disable_presence_on_nsfw = state.config.disable_presence_on_nsfw;

            if let Some(pres) = &mut state.presence {
                debug!("Setting Discord presence for game: {}", game.title);
                pres.set(DiscordGameDetails::new(
                    &game_id,
                    &game.title,
                    &game.image_url,
                    game.is_nsfw && disable_presence_on_nsfw,
                ))
                .map_err(|e| {
                    error!("Error setting Discord presence for {}: {:?}", game_id, e);
                    "Error setting presence".to_string()
                })?;
                info!("Discord presence set for game: {}", game.title);
            }

            debug!("Starting playtime tracking for {}", game_id);
            playtime::ClassicPlaytime::spawn(&app_handle);

            store.set_first_played(&game_id).map_err(|e| {
                error!("Error setting first played for {}: {:?}", game_id, e);
                "Error happened while setting first played"
            })?;

            app_handle
                .emit("current_game", json!({"id": game_id, "status": "playing"}))
                .map_err(|e| {
                    error!("Error emitting current_game event for {}: {:?}", game_id, e);
                    "Error happened while emitting"
                })?;

            info!("Game {} is now running and being tracked", game_id);

            Result::<(), String>::Ok(())
        });
    } else {
        error!("Game not found in store: {}", game_id);
        return Err("Game not found in store".to_string());
    }

    Ok(())
}

/// Gets a list of open windows
#[tauri::command]
pub fn get_active_windows() -> Result<Vec<ActiveWindow>, String> {
    debug!("Getting active windows list");

    #[cfg(windows)]
    {
        let open_windows = match x_win::get_open_windows() {
            Ok(open_windows) => {
                debug!("Found {} active windows", open_windows.len());
                open_windows
            }
            Err(x_win::XWinError) => {
                error!("Error occurred while getting open windows");
                return Err(String::from("error"));
            }
        };

        let active_windows: Vec<ActiveWindow> = open_windows
            .iter()
            .map(|window| {
                debug!("Processing window: {} ({})", window.title, window.info.path);
                ActiveWindow {
                    icon: x_win::get_window_icon(window).unwrap_or_default().data,
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
pub fn close_game(app_handle: AppHandle) -> Result<(), String> {
    info!("Attempting to close current game");

    let binding = app_handle.state::<Mutex<AppState>>();
    let state = binding.lock().map_err(|e| {
        error!("Error acquiring mutex lock for close_game: {:?}", e);
        "Error acquiring mutex lock".to_string()
    })?;

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

        // TODO: Uncomment these lines when ready to clear game state
        // debug!("Clearing game state and Discord presence");
        // state.game = None;

        // if let Some(pres) = &mut state.presence {
        //     pres.reset()
        //         .map_err(|_| "Error happened while clearing presence")?;
        // }

        // app_handle.emit("current_game", json!(null)).expect("here");
    }

    Ok(())
}
