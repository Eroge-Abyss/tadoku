use crate::scripts;
use crate::services::stores::games::GamesStore;
use std::{
    error::Error,
    fs,
    io::Cursor,
    path::{Path, PathBuf},
};
use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};
use tauri::{AppHandle, Manager};
use tauri_plugin_http::reqwest;
use url::Url;

/// Extracts an image filename from an image URL
pub fn extract_image(url: &str) -> Result<String, Box<dyn Error>> {
    let url = Url::parse(url).map_err(|_| "Failed to parse URL")?;
    Ok(url
        .path_segments()
        .ok_or("Failed to get segments")?
        .next_back()
        .ok_or("Failed to get filename")?
        .to_string())
}

pub fn construct_image_path(base_path: &Path, url: &str) -> Result<PathBuf, Box<dyn Error>> {
    Ok(base_path.join("images").join(extract_image(url)?))
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

/// Saves an image to storage
pub async fn save_image(app_handle: &AppHandle, image_url: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(image_url)
        .await
        .map_err(|_| "Failed to fetch image")?;

    let base_path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|err| err.to_string())?;

    scripts::create_images_folder(app_handle).map_err(|err| err.to_string())?;

    let path = construct_image_path(&base_path, image_url)
        .map_err(|_| "Failed to construct image path")?;

    let mut file = fs::File::create(&path).map_err(|err| err.to_string())?;
    let mut content = Cursor::new(response.bytes().await.map_err(|err| err.to_string())?);

    std::io::copy(&mut content, &mut file).map_err(|_| "Failed to download image")?;

    Ok(path.to_str().expect("Should not happen").to_owned())
}

/// Flushes playtime to disk
pub fn flush_playtime(
    app_handle: &AppHandle,
    game_id: &str,
    playtime: u64,
) -> Result<(), Box<dyn Error>> {
    let store = GamesStore::new(app_handle).map_err(|_| "Error happened while accessing store")?;

    store
        .update_playtime(game_id, playtime)
        .map_err(|_| "Error happened while setting new playtime")?;

    Ok(())
}

pub fn is_debug_mode() -> bool {
    std::env::var("TADOKU_DEBUG")
        .map(|s| s == "1" || s.to_lowercase() == "true")
        .unwrap_or(false)
}
