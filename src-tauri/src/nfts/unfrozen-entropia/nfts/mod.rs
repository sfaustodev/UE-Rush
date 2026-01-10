// Module for Unfrozen Entropia NFT collection
// This will contain logic for managing hundreds of NFTs in the collection

pub struct NftCollection {
    pub name: String,
    pub total_supply: u64,
    // Add more fields as needed
}

impl NftCollection {
    pub fn new() -> Self {
        Self {
            name: "Unfrozen Entropia".to_string(),
            total_supply: 1000, // Example, can be expanded to hundreds
        }
    }
}
