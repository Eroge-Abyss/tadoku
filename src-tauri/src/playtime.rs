use std::sync::Mutex;
use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};
use tauri::{AppHandle, Manager};
use tauri_plugin_shell::ShellExt;

use crate::AppState;

/// Opens a game and sets its PID in local state
#[tauri::command]
pub fn open_game(app_handle: AppHandle, exe_path: String) {
    tauri::async_runtime::block_on(async move {
        let (_, process) = app_handle
            .shell()
            .command(exe_path)
            .spawn()
            .expect("Error happened while running the game");

        {
            let state = app_handle.state::<Mutex<AppState>>();
            let mut state = state
                .lock()
                .expect("Error happened while acquiring mutex lock");

            state.game_pid = process.pid();
        }
    });
}

/// Gets the playtime of the current game in seconds
pub fn get_playtime(app_handle: &AppHandle) -> u64 {
    let state = app_handle.state::<Mutex<AppState>>();
    let state = state
        .lock()
        .expect("Error happened while acquiring mutex lock");

    // Is this bad for performance? to create a system instance on each call
    let s = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
    );

    let process = s
        .process(Pid::from(state.game_pid as usize))
        .expect("Couldn't find process");

    process.run_time()
}
