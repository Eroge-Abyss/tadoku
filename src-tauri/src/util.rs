use std::{error::Error, path::PathBuf};
use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};
use url::Url;

/// Extracts an image filename from an image URL
pub fn extract_image(url: &str) -> Result<String, Box<dyn Error>> {
    // TODO: Add proper error handling
    let url = Url::parse(url).map_err(|_| "Failed to parse URL")?;
    Ok(url
        .path_segments()
        .ok_or("Failed to get segments")?
        .last()
        .ok_or("Failed to get filename")?
        .to_string())
}

pub fn construct_image_path(base_path: &PathBuf, url: &str) -> Result<PathBuf, Box<dyn Error>> {
    Ok(base_path.join("images").join(extract_image(url)?))
}

/// Gets the playtime of the current game in seconds
pub fn get_playtime(pid: u32) -> Option<u64> {
    // Is this bad for performance? to create a system instance on each call
    let s = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
    );

    s.process(Pid::from(pid as usize))
        .map(|process| process.run_time())
}

/// Gets the PID of a saved game's process file path
pub fn get_pid_from_process_path(process_file_path: &str) -> Option<Pid> {
    let s = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
    );

    for (_, process) in s.processes() {
        if let Some(exe) = process.exe() {
            if exe.to_str().unwrap() == process_file_path {
                return Some(process.pid());
            }
        }
    }

    None
}
