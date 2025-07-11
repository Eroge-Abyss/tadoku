use crate::{
    services::{
        discord::DiscordGameDetails,
        playtime,
        stores::{games::GamesStore, settings::PlaytimeMode},
    },
    util, AppState, GameState,
};
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
    let lnk = lnk::ShellLink::open(&exe_path).map_err(|_| "Error opening .lnk file")?;

    let working_dir = lnk
        .working_dir()
        .as_ref()
        .ok_or("Missing working directory in .lnk file")?;
    let relative_path = lnk
        .relative_path()
        .as_ref()
        .ok_or("Missing relative path in .lnk file")?;
    *args = lnk
        .arguments()
        .as_ref()
        .unwrap_or(&String::new())
        .to_owned();
    *exe_path = fs::canonicalize(PathBuf::from(working_dir).join(relative_path))
        .map_err(|e| format!("Error resolving canonical path: {}", e))?;

    Ok(())
}

/// Opens a game and sets its PID in local state
#[tauri::command]
pub fn open_game(app_handle: AppHandle, game_id: String) -> Result<(), String> {
    let store =
        GamesStore::new(&app_handle).map_err(|e| format!("Error accessing store: {}", e))?;

    if let Some(game) = store.get(&game_id) {
        let mut exe_path = PathBuf::from(&game.exe_file_path);
        let mut args = String::new();

        if exe_path.extension().unwrap_or_default() == "lnk" {
            handle_open_lnk(&mut exe_path, &mut args)?;
        }

        tauri::async_runtime::block_on(async {
            let mut command = app_handle
                .shell()
                .command(&exe_path)
                .current_dir(exe_path.parent().ok_or("Failed to get parent directory")?);

            if !args.is_empty() {
                command = command.arg(args);
            }

            let _ = command.spawn().map_err(|e| {
                dbg!(e);
                "Error happened while running the game".to_string()
            })?;

            Result::<(), String>::Ok(())
        })?;

        tauri::async_runtime::spawn(async move {
            let binding = app_handle.state::<Mutex<AppState>>();
            let mut state = binding
                .lock()
                .map_err(|_| "Error acquiring mutex lock".to_string())?;

            let pid;
            let mut counter: u8 = 0;

            loop {
                counter += 1;
                match util::get_pid_from_process_path(&game.process_file_path) {
                    Some(found_pid) => {
                        pid = found_pid;
                        break;
                    }
                    None => {
                        if counter > 60 {
                            return Err(String::from("Couldn't find game process"));
                        }
                        thread::sleep(Duration::from_secs(1))
                    }
                }
            }

            state.game = Some(GameState {
                pid: pid.as_u32(),
                id: game_id.clone(),
                ..Default::default()
            });

            let disable_presence_on_nsfw = state.config.disable_presence_on_nsfw;

            if let Some(pres) = &mut state.presence {
                pres.set(DiscordGameDetails::new(
                    &game_id,
                    &game.title,
                    &game.image_url,
                    game.is_nsfw && disable_presence_on_nsfw,
                ))
                .map_err(|_| "Error setting presence".to_string())?;
            }

            match state.config.playtime_mode {
                PlaytimeMode::Classic => {
                    playtime::ClassicPlaytime::spawn(&app_handle);
                }
                PlaytimeMode::ExStatic => {
                    playtime::ExStaticPlaytime::spawn(&app_handle);
                }
            }

            store
                .set_first_played(&game_id)
                .map_err(|_| "Error happened while setting first played")?;

            app_handle
                .emit("current_game", json!({"id": game_id, "status": "playing"}))
                .map_err(|_| "Error happened while emitting")?;

            Result::<(), String>::Ok(())
        });
    } else {
        return Err("Game not found in store".to_string());
    }

    Ok(())
}

/// Gets a list of open windows
#[tauri::command]
pub fn get_active_windows() -> Result<Vec<ActiveWindow>, String> {
    #[cfg(windows)]
    {
        let open_windows = match x_win::get_open_windows() {
            Ok(open_windows) => open_windows,
            Err(x_win::XWinError) => {
                println!("error occurred while getting open windows");
                return Err(String::from("error"));
            }
        };

        Ok(open_windows
            .iter()
            .map(|window| ActiveWindow {
                icon: x_win::get_window_icon(window).unwrap().data,
                title: window.title.clone(),
                exe_path: window.info.path.clone(),
            })
            .collect())
    }

    #[cfg(not(windows))]
    Ok(Vec::new())
}

/// Closes a game and clears local state
#[tauri::command]
pub fn close_game(app_handle: AppHandle) -> Result<(), String> {
    let binding = app_handle.state::<Mutex<AppState>>();
    let mut state = binding
        .lock()
        .map_err(|_| "Error acquiring mutex lock".to_string())?;

    let mut system = System::new_all();
    system.refresh_all();

    if let Some(game) = &state.game {
        let pid = Pid::from_u32(game.pid);
        if let Some(process) = system.process(pid) {
            // Means the kill signal is successfully sent, doesn't mean the app has closed
            if process.kill() {
                process.wait();
            }
        }

        state.game = None;

        if let Some(pres) = &mut state.presence {
            pres.reset()
                .map_err(|_| "Error happened while clearing presence")?;
        }

        app_handle.emit("current_game", json!(null)).expect("here");
    }

    Ok(())
}
