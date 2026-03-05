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
        if let Some(filename) = parsed.path_segments().and_then(|s| s.last()) {
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
    // Check `file://` scheme first (fastest, no allocation).
    if url.starts_with("file://") {
        return true;
    }
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

/// Strips the `file://` (or `file:///`) scheme prefix from a URL, leaving a
/// usable filesystem path.  On Windows the leading `/` before the drive letter
/// is also removed.
pub fn strip_file_scheme(url: &str) -> &str {
    let path = url
        .strip_prefix("file:///")
        .or_else(|| url.strip_prefix("file://"))
        .unwrap_or(url);
    // On Windows paths look like /C:/... after stripping the scheme.
    #[cfg(windows)]
    let path = path.trim_start_matches('/');
    path
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

/// Saves an image to storage by downloading it from a remote URL.
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

/// Copies a local image file into the app images directory under `dest_filename`.
/// Returns the full destination path on success.
pub fn save_image_from_path(
    app_handle: &AppHandle,
    source_path: &str,
    dest_filename: &str,
) -> Result<String, Box<dyn Error>> {
    let base_path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|err| err.to_string())?;

    scripts::create_images_folder(app_handle).map_err(|err| err.to_string())?;

    let dest_path = base_path.join("images").join(dest_filename);
    fs::copy(source_path, &dest_path).map_err(|e| format!("Failed to copy local image: {e}"))?;

    Ok(dest_path.to_str().expect("Should not happen").to_owned())
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
        .update_chars_read(game_id, chars_read)
        .map_err(|_| "Error happened while setting chars_read")?;

    Ok(())
}

pub fn is_debug_mode() -> bool {
    std::env::var("TADOKU_DEBUG")
        .map(|s| s == "1" || s.to_lowercase() == "true")
        .unwrap_or(false)
}
