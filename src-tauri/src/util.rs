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

/// Extracts an image filename from an image URL or file path.
/// For HTTP(S) URLs, parses the URL and returns the last path segment.
/// For plain filenames (e.g. already-extracted names stored for local games),
/// returns the value itself after stripping any directory component.
pub fn extract_image(url: &str) -> Result<String, Box<dyn Error>> {
    if let Ok(parsed) = Url::parse(url) {
        if let Some(filename) = parsed.path_segments().and_then(|mut s| s.next_back()) {
            if !filename.is_empty() {
                return Ok(filename.to_string());
            }
        }
    }
    // Fall back to treating `url` as a file-system path / plain filename.
    let path = std::path::Path::new(url);
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .ok_or_else(|| "Failed to extract filename from path".into())
}

/// Returns `true` when `url` refers to a local file rather than a remote resource.
pub fn is_local_path(url: &str) -> bool {
    // Unix absolute path.
    if url.starts_with('/') {
        return true;
    }
    // Windows absolute path: drive letter followed by colon (e.g. `C:\...` or `C:/...`).
    #[cfg(windows)]
    if url.len() >= 3 && url.as_bytes()[1] == b':' {
        return true;
    }
    false
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

/// Saves an image to the images directory.
/// - `source`: HTTP(S) URL (downloaded) or local absolute path (copied).
/// - `dest_name`: filename to store as. When `None`, the name is derived from `source`.
pub async fn save_image(
    app_handle: &AppHandle,
    source: &str,
    dest_name: Option<&str>,
) -> Result<String, Box<dyn Error>> {
    let base_path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|err| err.to_string())?;

    scripts::create_images_folder(app_handle).map_err(|err| err.to_string())?;

    let filename = dest_name
        .map(str::to_owned)
        .unwrap_or_else(|| extract_image(source).unwrap_or_else(|_| "image.jpg".to_owned()));
    let dest = base_path.join("images").join(&filename);

    if is_local_path(source) {
        fs::copy(source, &dest).map_err(|e| format!("Failed to copy local image: {e}"))?;
    } else {
        let response = reqwest::get(source)
            .await
            .map_err(|_| "Failed to fetch image")?;
        let mut file = fs::File::create(&dest).map_err(|err| err.to_string())?;
        let mut content = Cursor::new(response.bytes().await.map_err(|err| err.to_string())?);
        std::io::copy(&mut content, &mut file).map_err(|_| "Failed to download image")?;
    }

    Ok(dest.to_str().expect("Should not happen").to_owned())
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

/// Flushes chars_read to disk
pub fn flush_chars_read(
    app_handle: &AppHandle,
    game_id: &str,
    chars_read: u64,
) -> Result<(), Box<dyn Error>> {
    let store = GamesStore::new(app_handle).map_err(|_| "Error happened while accessing store")?;

    store
        .update_game(game_id, |g| g.chars_read = chars_read)
        .map_err(|_| "Error happened while setting chars_read")?;

    Ok(())
}

pub fn is_debug_mode() -> bool {
    std::env::var("TADOKU_DEBUG")
        .map(|s| s == "1" || s.to_lowercase() == "true")
        .unwrap_or(false)
}
