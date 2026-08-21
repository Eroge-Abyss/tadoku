use crate::{
    prelude::{Fetchable, Result},
    services::{
        discord::DiscordGameDetails,
        playtime,
        state::{GameState, ManagedState},
        stores::games::{Game, GamesStore},
        system::SystemService,
    },
};
use anyhow::Context;
use lnk::encoding::WINDOWS_1252;
use log::{debug, error};
use serde_json::json;
use std::{path::PathBuf, time::Duration};
use sysinfo::{Pid, System};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_shell::ShellExt;

pub struct GameManager<'a> {
    app_handle: &'a AppHandle,
}

impl<'a> GameManager<'a> {
    pub fn new(app_handle: &'a AppHandle) -> Self {
        Self { app_handle }
    }

    pub fn open(&self, game_id: String) -> Result<()> {
        let store = GamesStore::new(self.app_handle)?;
        let game = store
            .get(&game_id)
            .context(format!("Game not found in store: {}", game_id))?;

        self.launch_process(&game)?;
        self.spawn_monitor(game_id, game);
        Ok(())
    }

    pub fn close(&self) -> Result<()> {
        let managed = self.app_handle.state::<ManagedState>();
        let state = managed.lock()?;

        if let Some(ref game) = state.game {
            let mut system = System::new_all();
            system.refresh_all();

            if let Some(process) = system.process(Pid::from_u32(game.pid)) {
                if process.kill() {
                    process.wait();
                }
            }
        }
        Ok(())
    }

    fn launch_process(&self, game: &Game) -> Result<()> {
        let mut exe_path = PathBuf::from(&game.exe_file_path);
        let mut args_str = String::new();

        if exe_path.extension().unwrap_or_default() == "lnk" {
            Self::handle_open_lnk(&mut exe_path, &mut args_str)?;
        }

        let current_dir = exe_path
            .parent()
            .context("Failed to get parent directory")?;
        let mut command = self
            .app_handle
            .shell()
            .command(&exe_path)
            .current_dir(current_dir);

        if !args_str.is_empty() {
            command = command.args(Self::split_args(&args_str));
        }

        command.spawn().context("Failed to spawn game process")?;
        Ok(())
    }

    fn spawn_monitor(&self, game_id: String, game: Game) {
        let app_handle = self.app_handle.clone();
        tauri::async_runtime::spawn(async move {
            let pid = match Self::find_pid(&game.process_file_path).await {
                Some(pid) => pid,
                None => {
                    error!("Timeout: couldn't find process for {}", game_id);
                    return;
                }
            };

            Self::set_game_state(&app_handle, &game_id, &game, pid);
            Self::start_tracking(&app_handle, &game_id);
        });
    }

    async fn find_pid(process_path: &str) -> Option<Pid> {
        for attempt in 1..=60u8 {
            if let Some(pid) = SystemService::get_pid_from_process_path(process_path) {
                return Some(pid);
            }
            if attempt == 60 {
                break;
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        None
    }

    fn set_game_state(app_handle: &AppHandle, game_id: &str, game: &Game, pid: Pid) {
        let managed = app_handle.state::<ManagedState>();
        let mut state = match managed.lock() {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to lock state: {}", e);
                return;
            }
        };

        state.game = Some(GameState {
            pid: pid.as_u32(),
            id: game_id.to_string(),
            chars_read: game.chars_read,
            ..Default::default()
        });

        let settings = state.settings.clone();

        if let Some(pres) = &mut state.presence {
            let title = match &game.alt_title {
                Fetchable::Available(alt) if settings.use_jp_for_title_time => alt.clone(),
                _ => game.title.clone(),
            };
            let _ = pres.set_presence(DiscordGameDetails {
                id: game_id.to_string(),
                title,
                image_url: game.image_url.clone(),
                nsfw_mode: game.is_nsfw && settings.disable_presence_on_nsfw,
                chars_read: game.chars_read,
                today_playtime: game.today_playtime,
            });
        }
    }

    fn start_tracking(app_handle: &AppHandle, game_id: &str) {
        let store = match GamesStore::new(app_handle) {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to access store: {}", e);
                return;
            }
        };

        playtime::ProcessMonitor::spawn(app_handle);

        if let Err(e) = store.set_first_played(game_id) {
            error!("Error setting first played for {}: {}", game_id, e);
        }

        if let Err(e) = app_handle.emit("current_game", json!({"id": game_id, "status": "playing"}))
        {
            error!("Error emitting current_game event: {}", e);
        }
    }

    fn handle_open_lnk(exe_path: &mut PathBuf, args: &mut String) -> anyhow::Result<()> {
        debug!("Handling .lnk file: {:?}", exe_path);

        let lnk = lnk::ShellLink::open(&exe_path, WINDOWS_1252)
            .context(format!("Error opening .lnk file {:?}", exe_path))?;

        *args = lnk
            .string_data()
            .command_line_arguments()
            .as_ref()
            .unwrap_or(&String::new())
            .to_owned();

        let mut resolved_path = None;

        if let Some(target) = lnk.link_target() {
            resolved_path = Some(PathBuf::from(target));
        }

        if resolved_path.is_none() {
            if let Some(rel_path) = lnk.string_data().relative_path() {
                if let Some(parent_dir) = exe_path.parent() {
                    resolved_path = Some(parent_dir.join(rel_path));
                }
            }
        }

        let final_path = resolved_path.context(format!(
            "Could not resolve a valid target path in .lnk file: {:?}",
            exe_path
        ))?;

        *exe_path = dunce::canonicalize(&final_path).context(format!(
            "Error resolving canonical path for: {:?}",
            final_path
        ))?;

        debug!(
            "Successfully processed .lnk file. Resolved exe_path: {:?}, args: {:?}",
            exe_path, args
        );

        Ok(())
    }

    fn split_args(args_str: &str) -> Vec<String> {
        #[cfg(target_os = "windows")]
        {
            winsplit::split(args_str)
        }

        #[cfg(not(target_os = "windows"))]
        {
            shell_words::split(args_str)
                .unwrap_or_else(|_| args_str.split_whitespace().map(|s| s.to_string()).collect())
        }
    }
}
