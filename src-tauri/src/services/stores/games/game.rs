use super::super::categories::Categories;
use super::character::Character;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Game {
    pub title: String,
    pub description: String,
    /// Is a local file path when loading games only, otherwise it's VNDB image URL
    pub image_url: String,
    pub exe_file_path: String,
    pub process_file_path: String,
    /// Play time in seconds
    pub playtime: u64,
    pub today_playtime: u64,
    pub last_played: Option<u64>,
    pub first_played: Option<u64>,
    pub last_play_date: Option<String>,
    pub is_pinned: bool,
    pub is_nsfw: bool,
    pub icon_url: Option<String>,
    pub notes: String,
    pub categories: Categories,
    // TODO: Make its own struct?
    pub characters: Option<Vec<Character>>,
}
