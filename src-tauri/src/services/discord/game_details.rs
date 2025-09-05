use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DiscordGameDetails<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub image_url: &'a str,
    pub nsfw_mode: bool,
}

impl<'a> DiscordGameDetails<'a> {
    pub fn new(id: &'a str, title: &'a str, image_url: &'a str, nsfw_mode: bool) -> Self {
        Self {
            id,
            title,
            image_url,
            nsfw_mode,
        }
    }
}
