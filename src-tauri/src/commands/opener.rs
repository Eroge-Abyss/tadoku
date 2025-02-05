use crate::{
    services::{discord::DiscordGameDetails, playtime, store::GamesStore},
    util, AppState, GameState,
};
use serde::Serialize;
use std::{
    fs::{self},
    path::PathBuf,
    sync::Mutex,
    thread,
    time::Duration,
};
use tauri::{AppHandle, Manager};
use tauri_plugin_shell::ShellExt;
use x_win::{XWinError};

#[derive(Serialize)]
pub struct ActiveWindow {
    title: String,
    exe_path: String,
    icon: String,
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
            let lnk = lnk::ShellLink::open(&exe_path).map_err(|_| "Error opening .lnk file")?;

            let working_dir = lnk
                .working_dir()
                .as_ref()
                .ok_or("Missing working directory in .lnk file")?;
            let relative_path = lnk
                .relative_path()
                .as_ref()
                .ok_or("Missing relative path in .lnk file")?;
            args = lnk
                .arguments()
                .as_ref()
                .unwrap_or(&String::new())
                .to_owned();
            exe_path = fs::canonicalize(PathBuf::from(working_dir).join(relative_path))
                .map_err(|e| format!("Error resolving canonical path: {}", e))?;
        }

        tauri::async_runtime::block_on(async move {
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

            {
                let state = app_handle.state::<Mutex<AppState>>();
                let mut state = state
                    .lock()
                    .map_err(|_| "Error acquiring mutex lock".to_string())?;

                // Fetch PID of actual game process
                let pid;

                loop {
                    match util::get_pid_from_process_path(&game.process_file_path) {
                        Some(found_pid) => {
                            pid = found_pid;
                            break;
                        }
                        None => thread::sleep(Duration::from_secs(1)),
                    }
                }

                state.game = Some(GameState {
                    pid: pid.as_u32(),
                    id: game_id.clone(),
                    ..Default::default()
                });

                if let Some(pres) = &mut state.presence {
                    pres.set(DiscordGameDetails::new(game_id, game.title, game.image_url))
                        .map_err(|_| "Error setting presence".to_string())?;
                }
            }

            playtime::spawn_playtime_thread(app_handle);
            Result::<(), String>::Ok(())
        })?;
    } else {
        return Err("Game not found in store".to_string());
    }

    Ok(())
}

/// Gets a list of open windows
#[tauri::command]
pub fn get_active_windows() -> Result<Vec<ActiveWindow>, String> {
    let open_windows = match x_win::get_open_windows() {
        Ok(open_windows) => {
            println!("open windows: {:#?}", open_windows);
            open_windows
        }
        Err(XWinError) => {
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
