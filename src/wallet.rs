use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::window;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Nft {
    pub name: String,
    pub image: String,
    pub description: String,
}

pub async fn invoke<T: serde::de::DeserializeOwned>(cmd: &str, args: &impl serde::Serialize) -> Result<T, JsValue> {
    let window = web_sys::window().unwrap();
    let tauri = js_sys::Reflect::get(&window, &JsValue::from_str("__TAURI__")).unwrap();
    let invoke_fn = js_sys::Reflect::get(&tauri, &JsValue::from_str("invoke")).unwrap();
    let args_js = serde_wasm_bindgen::to_value(args).unwrap();
    let promise = js_sys::Function::from(invoke_fn).call2(&JsValue::null(), &JsValue::from_str(cmd), &args_js).unwrap();
    let result = wasm_bindgen_futures::JsFuture::from(js_sys::Promise::from(promise)).await?;
    Ok(serde_wasm_bindgen::from_value(result).unwrap())
}

pub fn generate_seed_action() -> Rc<Action<(), Result<String, JsValue>>> {
    Rc::new(create_action(move |()| async move {
        invoke::<String>("generate_bip39_seed", &()).await
    }))
}

pub fn connect_wallet_action(expected_pubkey: ReadSignal<String>) -> Rc<Action<(), String>> {
    Rc::new(create_action(move |()| async move {
        // Connect to Phantom
        let window = web_sys::window().unwrap();
        let solana = js_sys::Reflect::get(&window, &"solana".into()).unwrap();
        if solana.is_undefined() {
            return "Phantom wallet not found. Please install Phantom.".to_string();
        }
        let is_phantom = js_sys::Reflect::get(&solana, &"isPhantom".into()).unwrap();
        if !is_phantom.is_truthy() {
            return "Phantom wallet not detected.".to_string();
        }
        let connect_fn = js_sys::Reflect::get(&solana, &"connect".into()).unwrap();
        if !connect_fn.is_function() {
            return "Connect function not available.".to_string();
        }
        let promise = connect_fn.dyn_ref::<js_sys::Function>().unwrap().call0(&solana).unwrap();
        match wasm_bindgen_futures::JsFuture::from(js_sys::Promise::from(promise)).await {
            Ok(response) => {
                let pubkey_obj = js_sys::Reflect::get(&response, &"publicKey".into()).unwrap();
                let to_string_fn = js_sys::Reflect::get(&pubkey_obj, &"toString".into()).unwrap();
                let pubkey_str = to_string_fn.dyn_ref::<js_sys::Function>().unwrap().call0(&pubkey_obj).unwrap().as_string().unwrap();
                // Compare with expected
                if pubkey_str == expected_pubkey.get() {
                    "Connected successfully. Public Key matches.".to_string()
                } else {
                    format!("Public Key mismatch. Expected: {}, Got: {}", expected_pubkey.get(), pubkey_str)
                }
            }
            Err(e) => format!("Failed to connect: {:?}", e),
        }
    }))
}

pub fn fetch_nfts_action() -> Rc<Action<(), Vec<Nft>>> {
    Rc::new(create_action(move |()| async move {
        if let Ok(nfts) = invoke::<Vec<Nft>>("fetch_nfts", &()).await {
            nfts
        } else {
            // Placeholder NFTs for demo
            vec![
                Nft {
                    name: "Sample NFT 1".to_string(),
                    image: "https://via.placeholder.com/150".to_string(),
                    description: "A sample NFT".to_string(),
                },
                Nft {
                    name: "Sample NFT 2".to_string(),
                    image: "https://via.placeholder.com/150".to_string(),
                    description: "Another sample NFT".to_string(),
                },
            ]
        }
    }))
}

pub fn check_soulbound_nft_action() -> Rc<Action<(), bool>> {
    Rc::new(create_action(move |()| async move {
        // Fetch NFTs and check for soulbound free_world NFT
        if let Some(nfts) = fetch_nfts_action().value().get() {
            // Placeholder check: look for NFT with name "Soulbound Free World"
            nfts.iter().any(|nft| nft.name == "Soulbound Free World")
        } else {
            false
        }
    }))
}

pub fn create_free_world_nft_action() -> Rc<Action<(), Result<String, JsValue>>> {
    Rc::new(create_action(move |()| async move {
        // Invoke Tauri command to create soulbound free_world NFT
        invoke::<String>("create_free_world_nft", &()).await
    }))
}

pub fn check_access_nft_action(world: String) -> Rc<Action<(), bool>> {
    Rc::new(create_action(move |()| async move {
        // Fetch NFTs and check for access NFT for the world
        if let Some(nfts) = fetch_nfts_action().value().get() {
            // Placeholder check: look for NFT with name "Access: {world}"
            nfts.iter().any(|nft| nft.name == format!("Access: {}", world))
        } else {
            false
        }
    }))
}
