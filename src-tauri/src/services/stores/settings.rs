use crate::{prelude::*, services::discord::DiscordPresenceMode};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

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
            accent_color: String::from("#2a2a2a"),
            use_custom_accent: false,
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SortOrder {
    Playtime,
    LastPlayed,
    #[default]
    Title,
}

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum PlaytimeMode {
    #[default]
    Classic,
    ExStatic,
}

// #[derive(Serialize, Deserialize)]
// struct Settings {
//     theme_settings: ThemeSettings,
//     disable_presence_on_nsfw: bool,
//     sort_order: SortOrder
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
        match self.store.get("theme_settings") {
            Some(v) => {
                let settings: ThemeSettings = serde_json::from_value(v)?;
                Ok(settings)
            }
            None => Ok(ThemeSettings::default()),
        }
    }

    pub fn set_theme_settings(&self, theme_settings: ThemeSettings) -> Result<()> {
        self.store
            .set("theme_settings", serde_json::json!(theme_settings));

        Ok(())
    }

    pub fn set_presence_on_nsfw(&self, to: bool) -> Result<()> {
        self.store.set("disable_presence_on_nsfw", json!(to));

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

    pub fn get_sort_order(&self) -> Result<SortOrder> {
        let v: SortOrder = serde_json::from_value(
            self.store
                .get("sort_order")
                .unwrap_or(json!(SortOrder::default())),
        )?;

        Ok(v)
    }

    pub fn set_sort_order(&self, new_sort_order: SortOrder) -> Result<()> {
        self.store.set("sort_order", json!(new_sort_order));

        Ok(())
    }

    pub fn get_show_random_picker(&self) -> Result<bool> {
        let v: bool =
            serde_json::from_value(self.store.get("show_random_picker").unwrap_or(json!(true)))?;

        Ok(v)
    }

    pub fn set_show_random_picker(&self, to: bool) -> Result<()> {
        self.store.set("show_random_picker", json!(to));

        Ok(())
    }

    pub fn get_discord_presence_mode(&self) -> Result<DiscordPresenceMode> {
        let v: DiscordPresenceMode = serde_json::from_value(
            self.store
                .get("discord_presence_mode")
                .unwrap_or(json!(DiscordPresenceMode::default())),
        )?;

        Ok(v)
    }

    pub fn set_discord_presence_mode(&self, to: DiscordPresenceMode) -> Result<()> {
        self.store.set("discord_presence_mode", json!(to));

        Ok(())
    }

    pub fn get_playtime_mode(&self) -> Result<PlaytimeMode> {
        let v: PlaytimeMode = serde_json::from_value(
            self.store
                .get("playtime_mode")
                .unwrap_or(json!(PlaytimeMode::default())),
        )?;

        Ok(v)
    }

    pub fn set_playtime_mode(&self, to: PlaytimeMode) -> Result<()> {
        self.store.set("playtime_mode", json!(to));

        Ok(())
    }
}
