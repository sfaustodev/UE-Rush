use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EpicMysteryBox {
    pub id: String,
    pub name: String,
    pub description: String,
    pub rarity: String,
    pub contents: Vec<String>, // List of possible NFT rewards
}

impl EpicMysteryBox {
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: "Epic Mystery Box".to_string(),
            description: "An epic mystery box containing high-value rewards".to_string(),
            rarity: "Epic".to_string(),
            contents: vec![
                "Epic NFT #1".to_string(),
                "Epic NFT #2".to_string(),
                "Epic NFT #3".to_string(),
            ],
        }
    }
}
