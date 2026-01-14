# UE-Rush

**Version 0.0.1**

Unfrozen Entropia: Rush - A Tauri-based desktop application for NFT management and gaming on Solana blockchain.

## Features

- Wallet integration with Phantom
- NFT inventory display
- Mystery box system for NFTs
- D&D Grid Game subproject
- Built with Rust, Leptos, and Tauri

## Prerequisites

- Rust (latest stable)
- Node.js and npm
- Trunk (for WASM builds)

## Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd ue-rush
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Build the frontend:
   ```bash
   npm run build-frontend
   ```

4. Run in development mode:
   ```bash
   npm run dev
   ```

## Building

To build for production:
```bash
npm run build
```

## Project Structure

- `src/` - Leptos frontend (WASM)
- `src-tauri/` - Tauri backend (Rust)
- `projects/dnd-grid-game/` - Bevy-based D&D game

## Contributing

Please read the TODO.md for current tasks and contributions.

## License

[Add license here]
