use crate::prelude::Result;
use log::debug;
use serde::Serialize;
use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};

#[derive(Serialize)]
pub struct ActiveWindow {
    pub title: String,
    pub exe_path: String,
    pub icon: Option<String>,
}

pub struct SystemService;

impl SystemService {
    pub fn get_active_windows() -> Result<Vec<ActiveWindow>> {
        debug!("Getting active windows list");

        #[cfg(windows)]
        {
            use log::info;

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

    /// Gets the PID of a saved game's process file path
    pub fn get_pid_from_process_path(process_file_path: &str) -> Option<Pid> {
        let s = System::new_with_specifics(
            RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
        );

        for process in s.processes().values() {
            if let Some(exe) = process.exe() {
                if exe.to_str()? == process_file_path {
                    return Some(process.pid());
                }

                #[cfg(not(windows))]
                {
                    let normalized_path = process
                        .cmd()
                        .iter()
                        .filter_map(|s| s.to_str())
                        .collect::<Vec<&str>>()
                        .join(" ")
                        .replace("\\", "/");

                    if normalized_path.contains(process_file_path) {
                        return Some(process.pid());
                    }
                }
            }
        }

        None
    }
}
