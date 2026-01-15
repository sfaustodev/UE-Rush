use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};
use std::rc::Rc;
use std::cell::RefCell;
use gloo::timers::callback::Interval;

mod wallet;
mod nft;

#[wasm_bindgen]
pub fn mount() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    mount_to_body(|| view! { <App/> });
}

#[component]
fn App() -> impl IntoView {
    let (seed, set_seed) = create_signal(String::new());
    let (expected_pubkey, set_expected_pubkey) = create_signal(String::new());
    let (timer, set_timer) = create_signal(666);
    let (timer_expired, set_timer_expired) = create_signal(false);
    let (connected, set_connected) = create_signal(false);
    let (loading, set_loading) = create_signal(false);
    let (nfts, set_nfts) = create_signal(Vec::<wallet::Nft>::new());
    let (has_soulbound, set_has_soulbound) = create_signal(false);

    let generate_seed = wallet::generate_seed_action();
    let connect_wallet = wallet::connect_wallet_action(expected_pubkey);
    let fetch_nfts = wallet::fetch_nfts_action();
    let check_soulbound = wallet::check_soulbound_nft_action();
    let create_free_world = wallet::create_free_world_nft_action();
    let start_bevy_game = wallet::start_bevy_game_action(wallet::PlayerData {
        position: (0.0, 0.0),
        skills: vec!["Basic Attack".to_string()], // Placeholder
        nfts: nfts.get(),
    });

    create_effect(move |_| {
        if let Some(Ok(data)) = generate_seed.value().get() {
            let parts: Vec<&str> = data.split('|').collect();
            if parts.len() == 2 {
                set_seed.set(parts[0].to_string());
                set_expected_pubkey.set(parts[1].to_string());
            }
            // Start timer
            let t = Rc::new(RefCell::new(666));
            set_timer_expired.set(false);
            let interval = Interval::new(1000, move || {
                let mut t_val = t.borrow_mut();
                *t_val -= 1;
                set_timer.set(*t_val);
                if *t_val <= 0 {
                    set_timer_expired.set(true);
                    // Lock UI
                    let _ = window().unwrap().alert_with_message("Time Expired. You can now connect to Phantom.");
                }
            });
            interval.forget();
        }
    });

    create_effect(move |_| {
        if let Some(msg) = connect_wallet.value().get() {
            if msg.contains("Connected successfully") {
                set_connected.set(true);
                set_loading.set(true);
                fetch_nfts.dispatch(());
            }
        }
    });

    create_effect(move |_| {
        if let Some(nfts_val) = fetch_nfts.value().get() {
            set_nfts.set(nfts_val.clone());
            // Check for soulbound free_world NFT
            let has = nfts_val.iter().any(|nft| nft.name == "Soulbound Free World");
            set_has_soulbound.set(has);
            if !has {
                // Create free_world NFT
                create_free_world.dispatch(());
            }
            set_loading.set(false);
        }
    });

    create_effect(move |_| {
        if let Some(Ok(_)) = create_free_world.value().get() {
            // Refresh NFTs after creation
            fetch_nfts.dispatch(());
        }
    });

    // Display seed on canvas
    create_effect(move |_| {
        if !seed.get().is_empty() {
            let canvas = window().unwrap().document().unwrap().get_element_by_id("seed-canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
            let ctx = canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();
            ctx.set_fill_style(&JsValue::from_str("#000"));
            ctx.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
            ctx.set_fill_style(&JsValue::from_str("#fff"));
            ctx.set_font("8px Arial");
            let words: Vec<String> = seed.get().split(' ').map(|s| s.to_string()).collect();
            for (i, word) in words.iter().enumerate() {
                ctx.fill_text(word, 10.0 + (i % 6) as f64 * 60.0, 20.0 + (i / 6) as f64 * 15.0).unwrap();
            }
            // Overlay
            ctx.set_fill_style(&JsValue::from_str("rgba(0,0,0,0.5)"));
            ctx.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        }
    });

    view! {
        <div>
            <h1>"UE-Rush Wallet"</h1>
            <Show
                when=move || seed.get().is_empty()
                fallback= || view! {
                    <Show
                        when=move || !timer_expired.get()
                        fallback= || view! {
                            <Show
                                when=move || !connected.get()
                                fallback= || view! {
                                    <Show
                                        when=move || loading.get()
                                        fallback= || view! {
                                            <div>
                                                <div>"Soulbound Free World NFT: " {move || format!("{}", if has_soulbound.get() { "Yes" } else { "Creating..." }) }</div>
                                                {nft::nft_display(nfts)}
                                            </div>
                                        }
                                    >
                                        <div>"Loading NFTs..."</div>
                                    </Show>
                                }
                            >
                                <div>
                                    <button on:click= |_| connect_wallet.dispatch(())>"Connect Phantom"</button>
                                    <div>{connect_wallet.value()}</div>
                                </div>
                            </Show>
                        }
                    >
                        <div>
                            <div>"Timer: " {timer}</div>
                            <div>"Warning: Do not screenshot or record this screen."</div>
                            <canvas id="seed-canvas" width="400" height="200"></canvas>
                        </div>
                    </Show>
                }
            >
                <button on:click= |_| generate_seed.dispatch(())>"Generate Seed"</button>
            </Show>
            <Show
                when=move || !loading.get() && connected.get() && has_soulbound.get()
                fallback=|| view! {}
            >
                <button on:click=move |_| {
                    start_bevy_game.dispatch(wallet::PlayerData {
                        position: (0.0, 0.0),
                        skills: vec!["Basic Attack".to_string()],
                        nfts: nfts.get(),
                    });
                    // TODO: Close or hide the Leptos UI
                }>"Entrar no Mundo"</button>
            </Show>
        </div>
    }
}
