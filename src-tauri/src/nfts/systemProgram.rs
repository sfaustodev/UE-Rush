use anchor_lang::prelude::*;
use mpl_token_metadata::instruction::{CreateMetadataAccountsV3, UpdateMetadataAccountsV2};

declare_id!("TeuProgramIdAqui111111111111111111111111111111");

pub const CREATOR_PUBKEY: Pubkey = pubkey!("TuaWalletCreatorAquiXxXxXxXxXxXxXxXxXxXxXxXx"); // HARDCODED - TU
pub const ROYALTY_BPS: u16 = 100; // 1% - // COMISSAO PARA O CRIADOR DO JOGO - 1% eterno, enforced, não divide nada

#[program]
pub mod unfrozen_entropia {
    use super::*;

    // Exemplo 1: Mint Entrópia Congelada - Grátis, Soulbound, Imortal
    pub fn mint_world_nft(ctx: Context<MintWorldNft>) -> Result<()> {
        let cpi_accounts = mpl_token_metadata::instruction::CreateMetadataAccountsV3 {
            metadata: ctx.accounts.metadata.key(),
            mint: ctx.accounts.mint.key(),
            mint_authority: ctx.accounts.mint_authority.key(),
            payer: ctx.accounts.payer.key(),
            update_authority: ctx.accounts.update_authority.key(),
            system_program: ctx.accounts.system_program.key(),
            rent: ctx.accounts.rent.key(),
        };
        let cpi_program = ctx.accounts.token_metadata_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        mpl_token_metadata::instruction::create_metadata_accounts_v3(
            cpi_ctx,
            "Entrópia Congelada".to_string(),
            "ENTROPIA".to_string(),
            "https://arweave.net/teu-json-vazio-preto-rachado".to_string(), // URI imagem preta rachada
            vec![mpl_token_metadata::state::Creator {
                address: CREATOR_PUBKEY,
                verified: true,
                share: 100, // 100% - // COMISSAO PARA O CRIADOR DO JOGO
            }],
            ROYALTY_BPS, // 1% enforced
            true, // update authority locked? false pra recovery
            true, // is_mutable false pra soulbound base
            None, // collection none
            None, // uses none
            None, // programmable ruleSet pra no-transfer (soulbound hook)
        )?;

        // Adiciona transfer hook pra bloquear venda eterna (soulbound)
        // ... CPI pra add transfer hook program que enforce no-transfer

        Ok(())
    }

    // Exemplo 2: Mint Palco do Vácuo - Espaço, Atributos Divertidos, Upável
    pub fn mint_combat_stage(ctx: Context<MintStage>, name: String, initial_uri: String) -> Result<()> {
        // Mesmo CPI create_metadata_v3
        // Traits iniciais no URI JSON: {"attributes": [{"trait_type": "Gravidade", "value": 0}, {"trait_type": "Temperatura", "value": 0}, ...]}
        // Royalties 1% enforced, creators 100% share pra CREATOR_PUBKEY
        // Mutable: true - pra update attributes depois

        // Depois, instruction separada: update_stage_attributes(level: u64, new_grav: i8, new_temp: i8, etc.)
        // CPI update_metadata_v2 pra mudar URI com novos traits + level on-chain

        Ok(())
    }

    // Exemplo 3: Mint Sopro Privado - Campanha Exclusiva, Grupos, Tolerância Noobs
    pub fn mint_campaign_nft(ctx: Context<MintCampaign>, name: String) -> Result<()> {
        // Mesmo pattern
        // Traits: {"attributes": [{"trait_type": "Tolerância Level", "value": 0}, {"trait_type": "Max Grupo", "value": 5}]}
        // Level up via instruction start_campaign_success - update metadata com +1 level, + tolerância
        // Taxa noobs: instruction pay_noob_tax - micro transação pra dono do NFT (on-chain escrow)

        // Royalties 1% em tudo - venda, taxas noobs, convites

        Ok(())
    }
}

// Accounts structs omitidos pra brevidade - padrão Anchor com Mint, Metadata, etc.