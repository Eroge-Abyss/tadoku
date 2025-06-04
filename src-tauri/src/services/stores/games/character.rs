use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    pub id: String,
    pub en_name: String,
    pub og_name: Option<String>,
    pub image_url: Option<String>,
}
