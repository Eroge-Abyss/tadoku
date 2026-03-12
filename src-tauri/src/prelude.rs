use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc};
use tauri::Wry;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub type Store = Arc<tauri_plugin_store::Store<Wry>>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum Fetchable<T> {
    /// The value has not been fetched yet.
    #[default]
    NotFetched,
    /// A fetch was attempted, but no value was found. We won't try again.
    NotFound,
    /// The value has been fetched and is available.
    Available(T),
}
