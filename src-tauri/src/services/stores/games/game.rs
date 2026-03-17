use super::super::categories::Categories;
use super::character::Character;
use crate::prelude::Fetchable;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Game {
    pub title: String,
    #[serde(default)]
    pub alt_title: Fetchable<String>,
    pub description: String,
    /// Is a local file path when loading games only, otherwise it's VNDB image URL
    pub image_url: String,
    pub exe_file_path: String,
    pub process_file_path: String,
    /// Play time in seconds
    #[serde(default)]
    pub playtime: u64,
    #[serde(default)]
    pub today_playtime: u64,
    pub last_played: Option<u64>,
    pub first_played: Option<u64>,
    pub last_play_date: Option<String>,
    #[serde(default)]
    pub is_pinned: bool,
    pub is_nsfw: bool,
    pub icon_url: Option<String>,
    #[serde(default)]
    pub notes: String,
    #[serde(default)]
    pub categories: Categories,
    // TODO: Make its own struct?
    pub characters: Option<Vec<Character>>,
    /// Cumulative characters read (from exSTATic)
    #[serde(default)]
    pub chars_read: u64,
    /// Total character count from Jiten API (pre-fetched at startup)
    #[serde(default)]
    pub jiten_char_count: Fetchable<u64>,
}
