use leptos::*;
use crate::wallet::Nft;

pub fn nft_display(nfts: ReadSignal<Vec<Nft>>) -> impl IntoView {
    view! {
        <div class="nft-inventory">
            <h2>"Your NFT Inventory"</h2>
            <div class="nft-grid">
                <For
                    each=move || nfts.get()
                    key=|nft| nft.name.clone()
                    children=move |nft| view! {
                        <div class="nft-card">
                            <img src={&nft.image} alt={&nft.name} />
                            <h3>{&nft.name}</h3>
                            <p>{&nft.description}</p>
                        </div>
                    }
                />
            </div>
        </div>
    }
}
