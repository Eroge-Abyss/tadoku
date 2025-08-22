mod game_details;
use crate::prelude::Result;
pub use crate::services::discord::game_details::DiscordGameDetails;
use discord_rich_presence::{
    activity::{Activity, Assets, Button, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::time;

const DISCORD_CLIENT_ID: &str = "1333425743572500490";

#[derive(Default, PartialEq, Eq, Deserialize, Serialize, Clone, Copy, Debug)]
pub enum DiscordPresenceMode {
    #[default]
    All,
    InGame,
    None,
}

pub struct DiscordPresence {
    client: DiscordIpcClient,
    mode: DiscordPresenceMode,
}

impl DiscordPresence {
    pub fn new(mode: DiscordPresenceMode) -> Result<Self> {
        info!("Initializing Discord presence with mode: {:?}", mode);
        let mut client = DiscordIpcClient::new(DISCORD_CLIENT_ID)?;

        client.connect().map_err(|e| {
            error!("Failed to connect to Discord: {}", e);
            e.to_string()
        })?;

        if mode == DiscordPresenceMode::All {
            info!("Setting default Discord activity (All mode)");
            client
                .set_activity(
                    Activity::new()
                        .state("In Menus")
                        .assets(Assets::new().large_image("app_icon").large_text("Tadoku")),
                )
                .map_err(|e| {
                    error!("Failed to set default Discord activity: {}", e);
                    e.to_string()
                })?;
        }

        info!("Discord presence initialized successfully");
        Ok(DiscordPresence { client, mode })
    }

    pub fn set(&mut self, details: DiscordGameDetails) -> Result<()> {
        debug!("Setting Discord activity: {:?}", details);
        if self.mode == DiscordPresenceMode::None {
            debug!("Discord presence mode is None, skipping set activity");
            return Ok(());
        }

        let url = format!("https://vndb.org/{}", details.id);
        debug!("VNDB URL: {}", url);

        let start = time::SystemTime::now();
        let since_the_epoch = start
            .duration_since(time::UNIX_EPOCH)
            .expect("Time went backwards");
        let unix_timestamp = since_the_epoch.as_secs();

        let assets = if details.nsfw_mode {
            Assets::new().large_image("app_icon").large_text("Tadoku")
        } else {
            Assets::new()
                .large_image(details.image_url)
                .large_text(details.title)
        };

        let buttons = if details.nsfw_mode {
            debug!("NSFW mode is enabled, hiding buttons");
            vec![]
        } else {
            debug!("NSFW mode is disabled, showing details button");
            vec![Button::new("Game Details", &url)]
        };

        self.client
            .set_activity(
                Activity::new()
                    .state(details.title)
                    .details("Playing")
                    .assets(assets)
                    .timestamps(Timestamps::new().start(unix_timestamp as i64))
                    .buttons(buttons),
            )
            .map_err(|e| {
                error!("Failed to set Discord activity: {}", e);
                e
            })
    }

    pub fn reset(&mut self) -> Result<()> {
        info!("Resetting Discord activity");
        if self.mode == DiscordPresenceMode::All {
            debug!("Discord presence mode is All, setting default activity");
            return self
                .client
                .set_activity(
                    Activity::new()
                        .state("In Menus")
                        .assets(Assets::new().large_image("app_icon").large_text("Tadoku")),
                )
                .map_err(|e| {
                    error!("Failed to set default Discord activity: {}", e);
                    e
                });
        } else {
            debug!("Discord presence mode is not All, clearing activity");
            self.client.clear_activity().map_err(|e| {
                error!("Failed to clear Discord activity: {}", e);
                e
            })?;
        }

        info!("Discord activity reset successfully");
        Ok(())
    }

    pub fn set_mode(&mut self, to: DiscordPresenceMode) {
        info!("Setting Discord presence mode to: {:?}", to);
        self.mode = to;
        match self.reset() {
            Ok(_) => info!("Discord presence mode set successfully"),
            Err(e) => error!("Failed to reset Discord presence: {}", e),
        };
    }
}
