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

    /// Normalizes a Windows-style path (as reported by WinAPI calls like
    /// QueryFullProcessImageNameW when running under Wine, e.g.
    /// "Z:\run\media\myhdd\mygame\game.exe") into the Unix-style path it
    /// actually corresponds to on disk (e.g. "/run/media/myhdd/mygame/game.exe").
    #[cfg(not(windows))]
    pub fn normalize_wine_path(path: &str) -> String {
        let bytes = path.as_bytes();
        let path_without_drive =
            if bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' {
                &path[2..]
            } else {
                path
            };
        path_without_drive.replace('\\', "/")
    }

    #[cfg(not(windows))]
    fn score_process(
        process: &sysinfo::Process,
        raw_needle: &str,
        norm_needle: &str,
        needle_lower: &str,
    ) -> Option<ProcessMatchQuality> {
        let exe_path_str = process.exe().and_then(|p| p.to_str()).unwrap_or("");
        let exe_name = process
            .exe()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("");
        let process_name = process.name().to_str().unwrap_or("");

        // Exact match on executable path (highest priority)
        if !exe_path_str.is_empty()
            && (exe_path_str == raw_needle
                || exe_path_str == norm_needle
                || exe_path_str.to_lowercase() == *needle_lower)
        {
            return Some(ProcessMatchQuality::ExactExe);
        }

        // Check command line arguments
        let normalized_cmd = process
            .cmd()
            .iter()
            .filter_map(|s| s.to_str())
            .collect::<Vec<&str>>()
            .join(" ")
            .replace('\\', "/");

        let cmd_lower = normalized_cmd.to_lowercase();

        if normalized_cmd.contains(norm_needle) || cmd_lower.contains(needle_lower) {
            let is_preloader = exe_name.contains("preloader")
                || process_name.contains("preloader")
                || exe_path_str.contains("preloader");

            if is_preloader {
                return Some(ProcessMatchQuality::WinePreloader);
            }

            let is_wrapper = matches!(
                exe_name,
                "wine"
                    | "wine64"
                    | "wineserver"
                    | "sh"
                    | "bash"
                    | "zsh"
                    | "dash"
                    | "gamemoderun"
                    | "mangohud"
                    | "env"
                    | "bwrap"
                    | "flatpak"
            ) || matches!(
                process_name,
                "wine"
                    | "wine64"
                    | "wineserver"
                    | "sh"
                    | "bash"
                    | "zsh"
                    | "dash"
                    | "gamemoderun"
                    | "mangohud"
            );

            if !is_wrapper {
                return Some(ProcessMatchQuality::CmdFuzzy);
            }
        }

        None
    }

    /// Gets the PID of a saved game's process file path
    pub fn get_pid_from_process_path(process_file_path: &str) -> Option<Pid> {
        if process_file_path.trim().is_empty() {
            return None;
        }

        let s = System::new_with_specifics(
            RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
        );

        #[cfg(windows)]
        {
            s.processes().values().find_map(|p| {
                let exe = p.exe()?.to_str()?;
                if exe.eq_ignore_ascii_case(process_file_path) {
                    Some(p.pid())
                } else {
                    None
                }
            })
        }

        #[cfg(not(windows))]
        {
            let normalized_needle = Self::normalize_wine_path(process_file_path);
            let needle_lower = normalized_needle.to_lowercase();

            s.processes()
                .values()
                .filter_map(|process| {
                    let quality = Self::score_process(
                        process,
                        process_file_path,
                        &normalized_needle,
                        &needle_lower,
                    )?;
                    Some((quality, process.pid()))
                })
                .max()
                .map(|(_, pid)| pid)
        }
    }
}

#[cfg(not(windows))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ProcessMatchQuality {
    /// Generic process whose command line contains the game path.
    CmdFuzzy,
    /// Wine preloader process (wine64-preloader / wine-preloader) executing the game.
    WinePreloader,
    /// Direct exact match on process executable path.
    ExactExe,
}
