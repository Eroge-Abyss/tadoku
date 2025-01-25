use crate::AppState;
use serde_json::json;
use std::{sync::Mutex, thread::sleep, time::Duration};
use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_shell::ShellExt;
use tauri_plugin_store::StoreExt;

/// Opens a game and sets its PID in local state
#[tauri::command]
pub fn open_game(app_handle: AppHandle, game_id: String) {
    let store = app_handle
        .store("store.json")
        .expect("Couldn't access games data");

    let games = store.get("gamesData").unwrap();

    if let Some(game) = games.get(&game_id) {
        let exe_path = game
            .get("exe_file_path")
            .expect("Couldn't find exe path")
            .as_str()
            .unwrap();

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

                state.game.pid = process.pid();
                state.game.id = game_id;
            }

            spawn_playtime_thread(app_handle);
        });
    }
}

/// Gets the playtime of the current game in seconds
fn get_playtime(pid: u32) -> Option<u64> {
    // Is this bad for performance? to create a system instance on each call
    let s = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
    );

    s.process(Pid::from(pid as usize))
        .map(|process| process.run_time())
}

/// Spawns a thread for playtime stats
fn spawn_playtime_thread(app_handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let state = app_handle.state::<Mutex<AppState>>();
        let mut state = state
            .lock()
            .expect("Error happened while acquiring mutex lock");

        loop {
            match get_playtime(state.game.pid) {
                Some(time) => {
                    state.game.current_playtime = time;
                    app_handle
                        .emit("play-time", time)
                        .expect("Error happened while emitting playtime");

                    sleep(Duration::from_secs(1));
                }
                None => {
                    let store = app_handle.store("store.json").expect("Couldn't open store");
                    let mut games_data = store.get("gamesData").unwrap();
                    let game = games_data.get_mut(&state.game.id).unwrap();

                    game["play_time"] =
                        json!(game["play_time"].as_u64().unwrap() + state.game.current_playtime);
                    store.set("gamesData", games_data);

                    break;
                }
            }
        }
    });
}
