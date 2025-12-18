use crate::{prelude::*, services::discord::DiscordPresenceMode};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeSettings {
    pub theme: String,
    pub accent_color: String,
    pub use_custom_accent: bool,
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            theme: String::from("default"),
            accent_color: String::from("#2a2a2a"),
            use_custom_accent: false,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum SortOrder {
    Playtime,
    LastPlayed,
    #[default]
    Title,
}

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case")]
pub enum PlaytimeMode {
    #[default]
    Classic,
    ExStatic,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub disable_presence_on_nsfw: bool,
    pub playtime_mode: PlaytimeMode,
    pub use_jp_for_title_time: bool,
    pub theme_settings: ThemeSettings,
    pub sort_order: SortOrder,
    pub show_random_picker: bool,
    pub discord_presence_mode: DiscordPresenceMode,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            disable_presence_on_nsfw: true,
            playtime_mode: PlaytimeMode::default(),
            use_jp_for_title_time: false,
            theme_settings: ThemeSettings::default(),
            sort_order: SortOrder::default(),
            show_random_picker: true,
            discord_presence_mode: DiscordPresenceMode::default(),
        }
    }
}

pub struct SettingsStore {
    app_handle: AppHandle,
}

impl SettingsStore {
    pub fn new(app_handle: &AppHandle) -> Result<Self> {
        Ok(Self {
            app_handle: app_handle.clone(),
        })
    }

    /// Loads settings from the store.
    /// Since the store is flat, we reconstruct the struct from individual keys.
    pub fn load(&self) -> Result<Settings> {
        debug!("Loading settings from store");
        let store = self.app_handle.store("settings.json")?;

        let default_settings = Settings::default();
        let default_value = serde_json::to_value(&default_settings)?;
        let default_obj = default_value
            .as_object()
            .expect("Settings must be an object");

        let mut map = serde_json::Map::new();

        for key in default_obj.keys() {
            if let Some(val) = store.get(key) {
                map.insert(key.clone(), val.clone());
            } else {
                map.insert(key.clone(), default_obj.get(key).unwrap().clone());
            }
        }

        let settings: Settings = serde_json::from_value(serde_json::Value::Object(map))?;
        Ok(settings)
    }

    /// Saves settings to the store as a flat structure.
    pub fn save(&self, settings: &Settings) -> Result<()> {
        info!("Saving settings to store");
        let store = self.app_handle.store("settings.json")?;
        let value = serde_json::to_value(settings)?;

        if let Some(obj) = value.as_object() {
            for (k, v) in obj {
                store.set(k.clone(), v.clone());
            }
        }

        store.save()?;
        Ok(())
    }
}
