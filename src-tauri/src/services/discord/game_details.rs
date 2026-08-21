#[derive(Debug, Clone)]
pub struct DiscordGameDetails {
    pub id: String,
    pub title: String,
    pub image_url: String,
    pub nsfw_mode: bool,
    pub show_chars: bool,
    pub chars_read: u64,
    pub today_playtime: u64,
}
