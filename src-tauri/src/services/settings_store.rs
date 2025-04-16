use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{error::Error, sync::Arc};
use tauri::{AppHandle, Wry};
use tauri_plugin_store::StoreExt;

type Store = Arc<tauri_plugin_store::Store<Wry>>;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize)]
pub struct ThemeSettings {
    theme: String,
    accent_color: String,
    use_custom_accent: bool,
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            theme: String::from("default"),
            accent_color: String::from("#3b82f6"),
            use_custom_accent: false,
        }
    }
}

// #[derive(Serialize, Deserialize)]
// struct Settings {
//     theme_settings: ThemeSettings,
//     disable_presence_on_nsfw: bool,
// }

// impl Default for Settings {
//     fn default() -> Self {
//         Self {
//             disable_presence_on_nsfw: true,
//             ..Default::default()
//         }
//     }
// }

pub struct SettingsStore {
    store: Store,
}

impl SettingsStore {
    pub fn new(app_handle: &AppHandle) -> Result<Self> {
        let store = app_handle.store("settings.json")?;

        Ok(Self { store })
    }

    pub fn get_theme_settings(&self) -> Result<ThemeSettings> {
        let theme_settings: ThemeSettings =
            serde_json::from_value(self.store.get("theme_settings").unwrap_or(json!({})))?;

        Ok(theme_settings)
    }

    pub fn set_theme_settings(&self, theme_settings: ThemeSettings) -> Result<()> {
        self.store
            .set("theme_settings", serde_json::json!(theme_settings));

        Ok(())
    }

    pub fn toggle_presence_on_nsfw(&self) -> Result<()> {
        let old: bool = serde_json::from_value(
            self.store
                .get("disable_presence_on_nsfw")
                .unwrap_or(json!(true)),
        )?;

        self.store.set("disable_presence_on_nsfw", !old);

        Ok(())
    }

    // TODO: Refactor
    pub fn get_presence_on_nsfw(&self) -> Result<bool> {
        let v: bool = serde_json::from_value(
            self.store
                .get("disable_presence_on_nsfw")
                .unwrap_or(json!(true)),
        )?;

        Ok(v)
    }
}
