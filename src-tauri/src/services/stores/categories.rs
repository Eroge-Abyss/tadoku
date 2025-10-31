use crate::prelude::*;
use log::{debug, info};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

pub type Categories = Vec<String>;

pub struct CategoriesStore {
    store: Store,
}

impl CategoriesStore {
    /// Creates store or uses existing one
    pub fn new(app_handle: &AppHandle) -> Result<Self> {
        info!("Creating CategoriesStore");
        let store = app_handle.store("store.json")?;

        Ok(Self { store })
    }

    fn get_store(&self) -> serde_json::Value {
        debug!("Getting categories from store");
        self.store
            .get("categories")
            .unwrap_or_else(|| serde_json::json!([]))
    }

    /// Gets all categories
    pub fn get_all(&self) -> Result<Categories> {
        debug!("Getting all categories");
        Ok(serde_json::from_value(self.get_store())?)
    }

    /// Sets categories array to the provided value
    pub fn set(&self, categories: Categories) -> Result<()> {
        info!("Setting categories to: {:?}", categories);
        self.store.set("categories", categories);

        Ok(())
    }

    /// Gets all selected categories
    pub fn get_selected(&self) -> Result<Categories> {
        debug!("Getting all selected categories");
        Ok(serde_json::from_value(
            self.store
                .get("selected_categories")
                .unwrap_or_else(|| serde_json::json!([])),
        )?)
    }

    /// Sets selected categories array to the provided value
    pub fn set_selected(&self, categories: Categories) -> Result<()> {
        info!("Setting selected categories to: {:?}", categories);
        self.store.set("selected_categories", categories);

        Ok(())
    }
}
