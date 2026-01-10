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

    let generate_seed = wallet::generate_seed_action();
    let connect_wallet = wallet::connect_wallet_action(expected_pubkey);
    let fetch_nfts = wallet::fetch_nfts_action();

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
            set_nfts.set(nfts_val);
            set_loading.set(false);
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
                fallback=|| view! {
                    <Show
                        when=move || !timer_expired.get()
                        fallback=|| view! {
                            <Show
                                when=move || !connected.get()
                                fallback=|| view! {
                                    <Show
                                        when=move || loading.get()
                                        fallback=|| view! { nft::nft_display(nfts) }
                                    >
                                        <div>"Loading NFTs..."</div>
                                    </Show>
                                }
                            >
                                <div>
                                    <button on:click=move |_| connect_wallet.dispatch(())>"Connect Phantom"</button>
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
                <button on:click=move |_| generate_seed.dispatch(())>"Generate Seed"</button>
            </Show>
        </div>
    }
}
