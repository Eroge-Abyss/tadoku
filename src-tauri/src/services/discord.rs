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
}

impl DiscordGameDetails {
    pub fn new(id: String, title: String, image_url: String) -> Self {
        Self {
            id,
            title,
            image_url,
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

        self.client.set_activity(
            Activity::new()
                .state(&details.title)
                .details("Playing")
                .assets(
                    Assets::new()
                        .large_image(&details.image_url)
                        .large_text(&details.title),
                )
                .timestamps(Timestamps::new().start(unix_timestamp as i64))
                .buttons(vec![Button::new("Game Details", &url)]),
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
