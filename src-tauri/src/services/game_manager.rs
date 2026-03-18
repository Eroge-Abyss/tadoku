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
        let mut args = String::new();

        if exe_path.extension().unwrap_or_default() == "lnk" {
            Self::handle_open_lnk(&mut exe_path, &mut args)?;
        }

        let current_dir = exe_path
            .parent()
            .context("Failed to get parent directory")?;
        let mut command = self
            .app_handle
            .shell()
            .command(&exe_path)
            .current_dir(current_dir);

        if !args.is_empty() {
            command = command.arg(args);
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
            ..Default::default()
        });

        let settings = state.settings.clone();

        if let Some(pres) = &mut state.presence {
            let title = match &game.alt_title {
                Fetchable::Available(alt) if settings.use_jp_for_title_time => alt.clone(),
                _ => game.title.clone(),
            };
            let _ = pres.set_presence(DiscordGameDetails::new(
                game_id,
                &title,
                &game.image_url,
                game.is_nsfw && settings.disable_presence_on_nsfw,
            ));
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

        playtime::ClassicPlaytime::spawn(app_handle);

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
        *exe_path = std::fs::canonicalize(PathBuf::from(working_dir).join(relative_path))
            .context("Error resolving canonical path for .lnk file")?;

        debug!(
            "Successfully processed .lnk file. Resolved exe_path: {:?}, args: {:?}",
            exe_path, args
        );
        Ok(())
    }
}
