use crate::prelude::*;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

pub type Categories = Vec<String>;

pub struct CategoriesStore {
    store: Store,
}

impl CategoriesStore {
    /// Creates store or uses existing one
    pub fn new(app_handle: &AppHandle) -> Result<Self> {
        let store = app_handle.store("store.json")?;

        Ok(Self { store })
    }

    fn get_store(&self) -> serde_json::Value {
        self.store
            .get("categories")
            .unwrap_or_else(|| serde_json::json!([]))
    }

    /// Gets all categories
    pub fn get_all(&self) -> Result<Categories> {
        Ok(serde_json::from_value(self.get_store())?)
    }

    /// Sets categories array to the provided value
    pub fn set(&self, categories: Categories) -> Result<()> {
        self.store.set("categories", categories);

        Ok(())
    }
}
