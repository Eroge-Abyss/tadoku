mod game_details;
use crate::prelude::Result;
pub use crate::services::discord::game_details::DiscordGameDetails;
use discord_rich_presence::{
    activity::{Activity, Assets, Button, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use serde::{Deserialize, Serialize};
use std::time;

const DISCORD_CLIENT_ID: &str = "1333425743572500490";

#[derive(Default, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
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
        let mut client = DiscordIpcClient::new(DISCORD_CLIENT_ID)?;

        client.connect()?;

        if mode == DiscordPresenceMode::All {
            client.set_activity(
                Activity::new()
                    .state("In Menus")
                    .assets(Assets::new().large_image("app_icon").large_text("Tadoku")),
            )?;
        }

        Ok(DiscordPresence { client, mode })
    }

    pub fn set(&mut self, details: DiscordGameDetails) -> Result<()> {
        if self.mode == DiscordPresenceMode::None {
            return Ok(());
        }

        let url = format!("https://vndb.org/{}", details.id);

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
            vec![]
        } else {
            vec![Button::new("Game Details", &url)]
        };

        self.client.set_activity(
            Activity::new()
                .state(details.title)
                .details("Playing")
                .assets(assets)
                .timestamps(Timestamps::new().start(unix_timestamp as i64))
                .buttons(buttons),
        )
    }

    pub fn reset(&mut self) -> Result<()> {
        if self.mode == DiscordPresenceMode::All {
            return self.client.set_activity(
                Activity::new()
                    .state("In Menus")
                    .assets(Assets::new().large_image("app_icon").large_text("Tadoku")),
            );
        } else {
            self.client.clear_activity()?;
        }

        Ok(())
    }

    pub fn set_mode(&mut self, to: DiscordPresenceMode) {
        self.mode = to;
        let _ = self.reset();
    }
}
