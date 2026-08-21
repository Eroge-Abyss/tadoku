mod game_details;
use crate::prelude::Result;
pub use crate::services::discord::game_details::DiscordGameDetails;
use anyhow::anyhow;
use discord_rich_presence::{
    DiscordIpc, DiscordIpcClient,
    activity::{Activity, Assets, Button, Timestamps},
};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};

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
        let mut client = DiscordIpcClient::new(DISCORD_CLIENT_ID);

        client
            .connect()
            .map_err(|e| anyhow!("Failed to connect to Discord IPC {e}"))?;

        if mode == DiscordPresenceMode::All {
            info!("Setting default Discord activity (All mode)");
            client
                .set_activity(
                    Activity::new()
                        .state("In Menus")
                        .assets(Assets::new().large_image("app_icon").large_text("Tadoku")),
                )
                .map_err(|e| anyhow!("{e}"))?;
        }

        info!("Discord presence initialized successfully");
        Ok(DiscordPresence { client, mode })
    }

    pub fn set_presence(&mut self, details: DiscordGameDetails) -> Result<()> {
        debug!("Setting Discord activity: {:?}", details);
        if self.mode == DiscordPresenceMode::None {
            debug!("Discord presence mode is None, skipping set activity");
            return Ok(());
        }

        let url = format!("https://vndb.org/{}", details.id);
        debug!("VNDB URL: {}", url);

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let today_start_timestamp = now.saturating_sub(details.today_playtime) as i64;

        let (assets, buttons) = if details.nsfw_mode {
            debug!("NSFW mode is enabled, hiding buttons");
            (
                Assets::new().large_image("app_icon").large_text("Tadoku"),
                vec![],
            )
        } else {
            debug!("NSFW mode is disabled, showing details button");
            (
                Assets::new()
                    .large_image(&details.image_url)
                    .large_text(&details.title),
                vec![Button::new("Game Details", &url)],
            )
        };

        let mut activity = Activity::new()
            .name(&details.title)
            .details("via Tadoku")
            .details_url(env!("CARGO_PKG_REPOSITORY"))
            .assets(assets)
            .timestamps(Timestamps::new().start(today_start_timestamp))
            .buttons(buttons);

        let state_text;
        if details.show_chars && details.chars_read > 0 {
            state_text = format!("{} chars read", Self::format_number(details.chars_read));
            activity = activity.state(&state_text);
        }

        self.client
            .set_activity(activity)
            .map_err(|e| anyhow!("Failed to set Discord activity: {e}"))
    }

    fn format_number(n: u64) -> String {
        let s = n.to_string();
        let bytes = s.as_bytes();
        let mut res = String::new();
        let len = bytes.len();
        for (idx, &b) in bytes.iter().enumerate() {
            if idx > 0 && (len - idx) % 3 == 0 {
                res.push(',');
            }
            res.push(b as char);
        }
        res
    }

    pub fn reset_presence(&mut self) -> Result<()> {
        info!("Resetting Discord activity");
        if self.mode == DiscordPresenceMode::All {
            debug!("Discord presence mode is All, setting default activity");
            self.client
                .set_activity(
                    Activity::new()
                        .state("In Menus")
                        .assets(Assets::new().large_image("app_icon").large_text("Tadoku")),
                )
                .map_err(|_| anyhow!("Failed to set default Discord activity"))?;
        } else {
            debug!("Discord presence mode is not All, clearing activity");
            self.client
                .clear_activity()
                .map_err(|_| anyhow!("Failed to clear Discord activity"))?;
        }

        info!("Discord activity reset successfully");
        Ok(())
    }

    pub fn set_mode(&mut self, to: DiscordPresenceMode) {
        info!("Setting Discord presence mode to: {:?}", to);
        self.mode = to;
        match self.reset_presence() {
            Ok(_) => info!("Discord presence mode set successfully"),
            Err(e) => error!("Failed to reset Discord presence: {}", e),
        };
    }
}
