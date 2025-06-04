use std::{error::Error, sync::Arc};
use tauri::Wry;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub type Store = Arc<tauri_plugin_store::Store<Wry>>;
