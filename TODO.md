# TODO: UE-Rush Project Tasks

## Completed
- [x] Organize project structure professionally
- [x] Fix compilation errors in dnd-grid-game
- [x] Rename projects/dnd-grid-game to projects/free_world
- [x] Update Cargo.toml and main.rs for free_world
- [x] Remove unused imports and variables
- [x] Ensure free_world compiles (with some warnings remaining)
- [x] Implement D&D 5e leveling, skills, magic points rules
- [x] Set up Rust DB for hash-CID pairs
- [x] Integrate character NFTs into wallet.rs and lib.rs
- [x] Refactor game to be click-based with button confirmations for all actions
- [x] Build minimal RPG-like UI/UX for characters and game interface (inspired by D&D games)
- [x] Prepare for Metaplex royalties enforced NFT implementation on Solana devnet
- [x] Test the game interface for basic functionality
- [x] Add loading screen after wallet confirmation
- [x] Implement NFT inventory fetching and display
- [x] Prepare folder structure for "Unfrozen Entropia" collection
- [x] Create subfolders for hundreds of NFTs
- [x] Define and structure 4 types of mystery boxes
- [x] Ensure modular design for easy expansion
- [x] Implement NFT placeholders for worlds (soulbound free_world NFT at wallet connection, access NFTs for different worlds)

## Pending
- [x] Implement D&D character NFTs with Solana mechanics, IPFS mutable data, enforced royalties
- [x] Create soulbound recruit NFT for initial character
- [x] Create non-soulbound class NFTs for each D&D 5e class
- [x] Implement Solana program for immutable character mechanics
- [x] Add IPFS integration for mutable JSON data (leveling, skills, magic points)


## Completed
- [x] Reorganized project structure into frontend/leptos and backend/bevy
- [x] Moved Leptos code to frontend/leptos/
- [x] Moved Bevy code to backend/bevy/
- [x] Added start_bevy_game Tauri command
- [x] Added "Entrar no Mundo" button in Leptos
- [x] Updated Cargo.toml files for new structure

## New
- [] Combat 100% Bevy - Integrate Benai for combat narration via HTTP or Tauri IPC
- [] Implement GameState enum in Bevy with Exploring and Combat modes
- [] Add pause and grid spawning for combat
- [] Connect combat to Benai AI for enemy actions
- [] 