use crate::prelude::Result;
use anyhow::{Context, bail};
use log::{debug, info};
use std::fs;
use tauri::{AppHandle, Manager};
use tauri_plugin_fs::FsExt;

/// Creates images folder if it doesn't exist
pub fn ensure_folder(app_handle: &AppHandle) -> Result<()> {
    info!("Creating images folder");

    let app_local_data_dir = app_handle
        .path()
        .app_local_data_dir()
        .context("Failed to get app local data directory")?;

    let path = app_local_data_dir.join("images");
    debug!("Images folder path: {:?}", path);

    if let Err(err) = fs::create_dir_all(&path) {
        if err.kind() != std::io::ErrorKind::AlreadyExists {
            bail!("Failed to create images directory {:?}: {:?}", path, err);
        } else {
            debug!("Images directory already exists: {:?}", path);
        }
    } else {
        info!("Successfully created images directory: {:?}", path);
    }

    let scope = app_handle.fs_scope();
    scope
        .allow_directory(&path, true)
        .context("Failed to allow images directory access")?;

    debug!("Images directory access granted to filesystem scope");
    Ok(())
}
