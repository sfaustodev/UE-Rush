use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommonMysteryBox {
    pub id: String,
    pub name: String,
    pub description: String,
    pub rarity: String,
    pub contents: Vec<String>, // List of possible NFT rewards
}

impl CommonMysteryBox {
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: "Common Mystery Box".to_string(),
            description: "A common mystery box containing basic rewards".to_string(),
            rarity: "Common".to_string(),
            contents: vec![
                "Basic NFT #1".to_string(),
                "Basic NFT #2".to_string(),
                "Basic NFT #3".to_string(),
            ],
        }
    }
}
