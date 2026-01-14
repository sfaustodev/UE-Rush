use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    program::invoke,
    rent::Rent,
    sysvar::Sysvar,
    declare_id,
};
use borsh::{BorshDeserialize, BorshSerialize};
use spl_token::{instruction as token_instruction, state::Account as TokenAccount};
use spl_associated_token_account::instruction as ata_instruction;
use mpl_token_metadata::{
    instruction as mpl_instruction,
    state::{Creator, DataV2, Metadata},
};
use lazy_static::lazy_static;
use std::collections::HashMap;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"); // Placeholder program ID

entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum RoyaltyNftInstruction {
    CreateRecruitNft {
        name: String,
        symbol: String,
        uri: String,
        hash: String,
    },
    CreateClassNft {
        class: String,
        name: String,
        symbol: String,
        uri: String,
        hash: String,
    },
    UpdateCharacterData {
        hash: String,
    },
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CharacterData {
    pub level: u8,
    pub skills: Vec<String>,
    pub magic_points: u32,
    pub hash: String,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = RoyaltyNftInstruction::try_from_slice(instruction_data)?;

    match instruction {
        RoyaltyNftInstruction::CreateRecruitNft { name, symbol, uri, hash } => {
            create_recruit_nft(program_id, accounts, name, symbol, uri, hash)
        }
        RoyaltyNftInstruction::CreateClassNft { class, name, symbol, uri, hash } => {
            create_class_nft(program_id, accounts, class, name, symbol, uri, hash)
        }
        RoyaltyNftInstruction::UpdateCharacterData { hash } => {
            update_character_data(program_id, accounts, hash)
        }
    }
}

fn create_recruit_nft(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    symbol: String,
    uri: String,
    hash: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let mint_account = next_account_info(account_info_iter)?;
    let token_account = next_account_info(account_info_iter)?;
    let owner = next_account_info(account_info_iter)?;
    let metadata_account = next_account_info(account_info_iter)?;
    let payer = next_account_info(account_info_iter)?;
    let rent = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let associated_token_program = next_account_info(account_info_iter)?;
    let mpl_program = next_account_info(account_info_iter)?;

    // Create mint
    invoke(
        &system_instruction::create_account(
            payer.key,
            mint_account.key,
            Rent::get()?.minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN as u64,
            token_program.key,
        ),
        &[payer.clone(), mint_account.clone(), system_program.clone()],
    )?;

    invoke(
        &token_instruction::initialize_mint(
            token_program.key,
            mint_account.key,
            owner.key,
            Some(owner.key), // Freeze authority for soulbound
            0,
        )?,
        &[mint_account.clone(), rent.clone()],
    )?;

    // Create associated token account
    invoke(
        &ata_instruction::create_associated_token_account(
            payer.key,
            owner.key,
            mint_account.key,
        ),
        &[payer.clone(), token_account.clone(), owner.clone(), mint_account.clone(), system_program.clone(), token_program.clone(), associated_token_program.clone()],
    )?;

    // Mint 1 token
    invoke(
        &token_instruction::mint_to(
            token_program.key,
            mint_account.key,
            token_account.key,
            owner.key,
            &[owner.key],
            1,
        )?,
        &[mint_account.clone(), token_account.clone(), owner.clone()],
    )?;

    // Freeze the token account to make it soulbound
    invoke(
        &token_instruction::freeze_account(
            token_program.key,
            token_account.key,
            mint_account.key,
            owner.key,
            &[owner.key],
        )?,
        &[token_account.clone(), mint_account.clone(), owner.clone()],
    )?;

    // Create metadata
    let creators = vec![Creator {
        address: *program_id,
        verified: true,
        share: 100,
    }];

    let data = DataV2 {
        name,
        symbol,
        uri,
        seller_fee_basis_points: 500, // 5% royalty
        creators: Some(creators),
        collection: None,
        uses: None,
    };

    invoke(
        &mpl_instruction::create_metadata_accounts_v3(
            *mpl_program.key,
            *metadata_account.key,
            *mint_account.key,
            *owner.key,
            *payer.key,
            *program_id,
            data.name.clone(),
            data.symbol.clone(),
            data.uri.clone(),
            Some(data.creators.unwrap()),
            data.seller_fee_basis_points,
            true,
            true,
            None,
            None,
            None,
        ),
        &[metadata_account.clone(), mint_account.clone(), owner.clone(), payer.clone(), program_id.clone(), system_program.clone(), rent.clone()],
    )?;

    // Store hash
    store_hash_cid_pair(hash, "".to_string()); // CID will be set later

    msg!("Soulbound recruit NFT created");
    Ok(())
}

fn create_class_nft(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    class: String,
    name: String,
    symbol: String,
    uri: String,
    hash: String,
) -> ProgramResult {
    // Similar to recruit, but without freeze
    let account_info_iter = &mut accounts.iter();

    let mint_account = next_account_info(account_info_iter)?;
    let token_account = next_account_info(account_info_iter)?;
    let owner = next_account_info(account_info_iter)?;
    let metadata_account = next_account_info(account_info_iter)?;
    let payer = next_account_info(account_info_iter)?;
    let rent = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let associated_token_program = next_account_info(account_info_iter)?;
    let mpl_program = next_account_info(account_info_iter)?;

    // Create mint
    invoke(
        &system_instruction::create_account(
            payer.key,
            mint_account.key,
            Rent::get()?.minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN as u64,
            token_program.key,
        ),
        &[payer.clone(), mint_account.clone(), system_program.clone()],
    )?;

    invoke(
        &token_instruction::initialize_mint(
            token_program.key,
            mint_account.key,
            owner.key,
            None, // No freeze authority
            0,
        )?,
        &[mint_account.clone(), rent.clone()],
    )?;

    // Create associated token account
    invoke(
        &ata_instruction::create_associated_token_account(
            payer.key,
            owner.key,
            mint_account.key,
        ),
        &[payer.clone(), token_account.clone(), owner.clone(), mint_account.clone(), system_program.clone(), token_program.clone(), associated_token_program.clone()],
    )?;

    // Mint 1 token
    invoke(
        &token_instruction::mint_to(
            token_program.key,
            mint_account.key,
            token_account.key,
            owner.key,
            &[owner.key],
            1,
        )?,
        &[mint_account.clone(), token_account.clone(), owner.clone()],
    )?;

    // Create metadata with royalties
    let creators = vec![Creator {
        address: *program_id,
        verified: true,
        share: 100,
    }];

    let data = DataV2 {
        name,
        symbol,
        uri,
        seller_fee_basis_points: 500, // 5% royalty
        creators: Some(creators),
        collection: None,
        uses: None,
    };

    invoke(
        &mpl_instruction::create_metadata_accounts_v3(
            *mpl_program.key,
            *metadata_account.key,
            *mint_account.key,
            *owner.key,
            *payer.key,
            *program_id,
            data.name.clone(),
            data.symbol.clone(),
            data.uri.clone(),
            Some(data.creators.unwrap()),
            data.seller_fee_basis_points,
            true,
            true,
            None,
            None,
            None,
        ),
        &[metadata_account.clone(), mint_account.clone(), owner.clone(), payer.clone(), program_id.clone(), system_program.clone(), rent.clone()],
    )?;

    // Store hash
    store_hash_cid_pair(hash, "".to_string());

    msg!("Class NFT created: {}", class);
    Ok(())
}

fn update_character_data(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    hash: String,
) -> ProgramResult {
    // Update the hash for mutable data
    // In practice, this would update the metadata URI or store in PDA
    store_hash_cid_pair(hash, "".to_string());
    msg!("Character data updated");
    Ok(())
}

// Placeholder for hash-CID storage (use PDA in real implementation)
lazy_static! {
    static ref HASH_CID_MAP: std::sync::Mutex<HashMap<String, String>> = std::sync::Mutex::new(HashMap::new());
}

fn store_hash_cid_pair(hash: String, cid: String) {
    let mut map = HASH_CID_MAP.lock().unwrap();
    map.insert(hash, cid);
    msg!("Stored hash-CID pair: {} -> {}", hash, cid);
}

fn get_cid_for_hash(hash: &str) -> Option<String> {
    let map = HASH_CID_MAP.lock().unwrap();
    map.get(hash).cloned()
}
