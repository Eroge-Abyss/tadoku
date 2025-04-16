use discord_rich_presence::{
    activity::{Activity, Assets, Button, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use serde::Deserialize;
use std::{error::Error, time};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Deserialize)]
pub struct DiscordGameDetails {
    pub id: String,
    pub title: String,
    pub image_url: String,
    pub nsfw_mode: bool,
}

impl DiscordGameDetails {
    pub fn new(id: String, title: String, image_url: String, nsfw_mode: bool) -> Self {
        Self {
            id,
            title,
            image_url,
            nsfw_mode,
        }
    }
}

pub struct DiscordPresence {
    client: DiscordIpcClient,
}

impl DiscordPresence {
    pub fn new() -> Result<Self> {
        let mut client = DiscordIpcClient::new("1333425743572500490")?;

        client.connect()?;
        client.set_activity(
            Activity::new()
                .state("In Menus")
                .assets(Assets::new().large_image("app_icon").large_text("Tadoku")),
        )?;

        Ok(DiscordPresence { client })
    }

    pub fn set(&mut self, details: DiscordGameDetails) -> Result<()> {
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
                .large_image(&details.image_url)
                .large_text(&details.title)
        };
        let buttons = if details.nsfw_mode {
            vec![]
        } else {
            vec![Button::new("Game Details", &url)]
        };

        self.client.set_activity(
            Activity::new()
                .state(&details.title)
                .details("Playing")
                .assets(assets)
                .timestamps(Timestamps::new().start(unix_timestamp as i64))
                .buttons(buttons),
        )
    }

    pub fn clear(&mut self) -> Result<()> {
        self.client.set_activity(
            Activity::new()
                .state("In Menus")
                .assets(Assets::new().large_image("app_icon").large_text("Tadoku")),
        )
    }
}
