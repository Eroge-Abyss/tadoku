use crate::prelude::Result;
use anyhow::Context;
use std::{
    fs,
    io::Cursor,
    path::{Path, PathBuf},
};
use tauri::{AppHandle, Manager};
use tauri_plugin_http::reqwest;
use url::Url;

/// Extracts an image filename from an image URL or file path.
/// For HTTP(S) URLs, parses the URL and returns the last path segment.
/// For plain filenames (e.g. already-extracted names stored for local games),
/// returns the value itself after stripping any directory component.
pub fn extract_image(url: &str) -> Result<String> {
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
        .context("Failed to extract filename from path")
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

pub fn construct_image_path(base_path: &Path, url: &str) -> Result<PathBuf> {
    Ok(base_path.join("images").join(extract_image(url)?))
}

/// Saves an image to the images directory.
/// - `source`: HTTP(S) URL (downloaded) or local absolute path (copied).
/// - `dest_name`: filename to store as. When `None`, the name is derived from `source`.
pub async fn save_image(
    app_handle: &AppHandle,
    source: &str,
    dest_name: Option<&str>,
) -> Result<String> {
    let base_path = app_handle.path().app_local_data_dir()?;

    let filename = dest_name
        .map(str::to_owned)
        .unwrap_or_else(|| extract_image(source).unwrap_or_else(|_| "image.jpg".to_owned()));
    let dest = base_path.join("images").join(&filename);

    if is_local_path(source) {
        fs::copy(source, &dest).context("Failed to copy local image")?;
    } else {
        let response = reqwest::get(source)
            .await
            .context("Failed to fetch image")?;
        let mut file = fs::File::create(&dest)?;
        let mut content = Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut file).context("Failed to download image")?;
    }

    Ok(dest.to_str().expect("Should not happen").to_owned())
}
