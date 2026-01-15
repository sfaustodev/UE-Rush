use tauri::command;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use tts::*;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use reqwest;
use rand::Rng;
use whatlanggo::detect;

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

#[command]
pub fn start_bevy_game(player_data: String) -> String {
    use std::process::Command;
    let mut child = Command::new("./backend/bevy/target/debug/bevy-backend")
        .arg(player_data)
        .spawn()
        .expect("Failed to start Bevy game");
    // Note: In production, handle the child process properly, perhaps store it.
    "Bevy game started".to_string()
}

#[command]
pub async fn speak_text(text: String) -> String {
    let mut tts = Tts::default().unwrap();
    tts.speak(text, false).unwrap();
    "Speaking".to_string()
}

#[command]
pub fn roll_dice(sides: u32, count: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    (0..count).map(|_| rng.gen_range(1..=sides)).collect()
}

#[command]
pub async fn call_venice_api(prompt: String) -> String {
    let api_key = std::env::var("VENICE_API_KEY").unwrap_or_default();
    let client = reqwest::Client::new();
    let res = client.post("https://api.venice.ai/chat/completions") // Assuming endpoint
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({"model": "venice-uncensored", "messages": [{"role": "user", "content": prompt}]}))
        .send()
        .await;
    match res {
        Ok(resp) => resp.text().await.unwrap_or("Error".to_string()),
        Err(_) => "API call failed".to_string(),
    }
}

#[command]
pub async fn call_zai_api(prompt: String) -> String {
    let api_key = std::env::var("ZAI_API_KEY").unwrap_or_default();
    let client = reqwest::Client::new();
    let res = client.post("https://api.zai.ai/chat/completions") // Assuming endpoint
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({"model": "zai-guardian", "messages": [{"role": "user", "content": prompt}]}))
        .send()
        .await;
    match res {
        Ok(resp) => resp.text().await.unwrap_or("Error".to_string()),
        Err(_) => "API call failed".to_string(),
    }
}

#[command]
pub fn detect_language(text: String) -> String {
    let info = detect(&text);
    info.lang().to_string()
}
