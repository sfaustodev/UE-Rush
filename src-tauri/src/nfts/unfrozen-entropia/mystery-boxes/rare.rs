use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RareMysteryBox {
    pub id: String,
    pub name: String,
    pub description: String,
    pub rarity: String,
    pub contents: Vec<String>, // List of possible NFT rewards
}

impl RareMysteryBox {
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: "Rare Mystery Box".to_string(),
            description: "A rare mystery box containing valuable rewards".to_string(),
            rarity: "Rare".to_string(),
            contents: vec![
                "Rare NFT #1".to_string(),
                "Rare NFT #2".to_string(),
                "Rare NFT #3".to_string(),
            ],
        }
    }
}
