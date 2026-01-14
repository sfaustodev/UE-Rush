use tauri::command;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static KEYPAIR: Lazy<Mutex<Option<solana_sdk::signer::keypair::Keypair>>> = Lazy::new(|| Mutex::new(None));

#[command]
pub fn generate_bip39_seed() -> String {
    use bip39::{Mnemonic, MnemonicType, Language};
    use solana_sdk::signature::{Keypair, Signer};

    let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
    let seed = mnemonic.to_seed("");
    let keypair = Keypair::from_seed(&seed[..32]).unwrap(); // Derive keypair from seed

    let pubkey = keypair.pubkey().to_string();
    *KEYPAIR.lock().unwrap() = Some(keypair);

    // Return mnemonic and pubkey separated by |
    format!("{}|{}", mnemonic.phrase(), pubkey)
}

#[command]
pub fn get_public_key() -> String {
    if let Some(keypair) = KEYPAIR.lock().unwrap().as_ref() {
        keypair.pubkey().to_string()
    } else {
        "".to_string()
    }
}

#[command]
pub fn connect_phantom() -> String {
    // Placeholder for Phantom connection - actual connection is in frontend
    "Connection initiated from frontend".to_string()
}



#[command]
pub fn sign_message(message: String) -> String {
    if let Some(keypair) = KEYPAIR.lock().unwrap().as_ref() {
        use solana_sdk::signature::Signer;
        let signature = keypair.sign_message(message.as_bytes());
        signature.to_string()
    } else {
        "No keypair".to_string()
    }
}

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Nft {
    pub name: String,
    pub image: String,
    pub description: String,
}

#[command]
pub fn fetch_nfts() -> Vec<Nft> {
    // Placeholder for fetching NFTs from Metaplex
    // Use solana-client to query token accounts, then metaplex to get metadata
    vec![
        Nft {
            name: "Unfrozen Entropia #1".to_string(),
            image: "https://via.placeholder.com/150".to_string(),
            description: "First NFT in the collection".to_string(),
        },
        Nft {
            name: "Unfrozen Entropia #2".to_string(),
            image: "https://via.placeholder.com/150".to_string(),
            description: "Second NFT in the collection".to_string(),
        },
    ]
}

#[command]
pub fn create_recruit_nft() -> String {
    // Implement creating soulbound recruit NFT
    // Upload character data to IPFS, get CID, then call Solana program
    // For now, simulate
    "Soulbound Recruit NFT created successfully".to_string()
}

#[command]
pub fn create_free_world_nft() -> String {
    // Placeholder for creating soulbound free_world NFT
    // This would interact with the Solana program to mint the NFT
    // For now, return a success message
    "Soulbound Free World NFT created successfully".to_string()
}
