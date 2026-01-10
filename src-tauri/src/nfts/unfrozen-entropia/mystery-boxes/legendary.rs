use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LegendaryMysteryBox {
    pub id: String,
    pub name: String,
    pub description: String,
    pub rarity: String,
    pub contents: Vec<String>, // List of possible NFT rewards
}

impl LegendaryMysteryBox {
    pub fn new(id: String) -> Self {
        Self {
            id,
            name: "Legendary Mystery Box".to_string(),
            description: "A legendary mystery box containing the rarest rewards".to_string(),
            rarity: "Legendary".to_string(),
            contents: vec![
                "Legendary NFT #1".to_string(),
                "Legendary NFT #2".to_string(),
                "Legendary NFT #3".to_string(),
            ],
        }
    }
}
