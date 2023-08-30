use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3, Metadata},
    token::{burn, mint_to, Burn, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::{pda::find_metadata_account, state::DataV2};
use solana_program::{pubkey, pubkey::Pubkey};

declare_id!("JkTuLCESE1jLwBrGHBdbv1qx8oTsD5FHw4EgMLvbHam");

const ADMIN_PUBKEY: Pubkey = pubkey!("FEjRfGokWnz3tpHj38p8ktgWsPGiemn5sNcgubS8xsp8");

#[program]
pub mod marmot_network {
   use super::*;

    // Create new token mint with PDA as mint authority
   pub fn create_mint(
       ctx: Context<CreateMint>,
       uri: String,
       name: String,
       symbol: String,
   ) -> Result<()> {
       // PDA seeds and bump to "sign" for CPI
       let seeds = b"reward";
       let bump = *ctx.bumps.get("reward_token_mint").unwrap();
       let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];

       // On-chain token metadata for the mint
       let data_v2 = DataV2 {
           name: name,
           symbol: symbol,
           uri: uri,
           seller_fee_basis_points: 0,
           creators: None,
           collection: None,
           uses: None,
       };

       // CPI Context
       let cpi_ctx = CpiContext::new_with_signer(
           ctx.accounts.token_metadata_program.to_account_info(),
           CreateMetadataAccountsV3 {
                // the metadata account being created
               metadata: ctx.accounts.metadata_account.to_account_info(),
                // the mint account of the metadata account
               mint: ctx.accounts.reward_token_mint.to_account_info(),
               // the mint authority of the mint account
               mint_authority: ctx.accounts.reward_token_mint.to_account_info(),
               // the update authority of the metadata account
               update_authority: ctx.accounts.reward_token_mint.to_account_info(),
               // the payer for creating the metadata account
               payer: ctx.accounts.admin.to_account_info(),
               // the system program account
               system_program: ctx.accounts.system_program.to_account_info(),
               // the rent sysvar account
               rent: ctx.accounts.rent.to_account_info(),
           },
           signer,
       );

       create_metadata_accounts_v3(
           cpi_ctx, // cpi context
           data_v2, // token metadata
           true,    // is_mutable
           true,    // update_authority_is_signer
           None,    // collection details
       )?;

       Ok(())
   }

   // Create new player account
   pub fn init_player(ctx: Context<InitPlayer>) -> Result<()> {
       ctx.accounts.player_data.win = 0;
       ctx.accounts.player_data.lose = 0;
       Ok(())
   }

   pub fn guess_one(ctx: Context<GuessOne>) -> Result<()> {
       // Get current slot
       let slot = Clock::get()?.slot;
       // Generate pseudo-random number using XORShift with the current slot as seed
       let xorshift_output = xorshift64(slot);
       // Calculate random result
       let random_result = xorshift_output % 2 + 1;

       // PDA seeds and bump to "sign" for CPI
       let seeds = b"reward";
       let bump = *ctx.bumps.get("reward_token_mint").unwrap();
       let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];

       if random_result == 1 {

           ctx.accounts.player_data.win = ctx.accounts.player_data.win.checked_add(1).unwrap();
           // CPI Context
           let cpi_ctx = CpiContext::new_with_signer(
               ctx.accounts.token_program.to_account_info(),
               MintTo {
                   mint: ctx.accounts.reward_token_mint.to_account_info(),
                   to: ctx.accounts.player_token_account.to_account_info(),
                   authority: ctx.accounts.reward_token_mint.to_account_info(),
               },
               signer,
           );

           // Mint 1 token, accounting for decimals of mint
           let amount = (1u64)
               .checked_mul(10u64.pow(ctx.accounts.reward_token_mint.decimals as u32))
               .unwrap();

           mint_to(cpi_ctx, amount)?;
        } else {

            ctx.accounts.player_data.lose = ctx.accounts.player_data.lose.checked_add(1).unwrap();
            // CPI Context
            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Burn {
                    mint: ctx.accounts.reward_token_mint.to_account_info(),
                    from: ctx.accounts.player_token_account.to_account_info(),
                    authority: ctx.accounts.player.to_account_info(),
                },
            );

            // Burn 1 token, accounting for decimals of mint
            let amount = (1u64)
                .checked_mul(10u64.pow(ctx.accounts.reward_token_mint.decimals as u32))
                .unwrap();

            burn(cpi_ctx, amount)?;
        }

       Ok(())
   }

   pub fn guess_two(ctx: Context<GuessTwo>) -> Result<()> {
       // Get current slot
       let slot = Clock::get()?.slot;
       // Generate pseudo-random number using XORShift with the current slot as seed
       let xorshift_output = xorshift64(slot);
       // Calculate random result
       let random_result = xorshift_output % 2 + 1;

       // PDA seeds and bump to "sign" for CPI
       let seeds = b"reward";
       let bump = *ctx.bumps.get("reward_token_mint").unwrap();
       let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];

       if random_result == 2 {

           ctx.accounts.player_data.win = ctx.accounts.player_data.win.checked_add(1).unwrap();
           // CPI Context
           let cpi_ctx = CpiContext::new_with_signer(
               ctx.accounts.token_program.to_account_info(),
               MintTo {
                   mint: ctx.accounts.reward_token_mint.to_account_info(),
                   to: ctx.accounts.player_token_account.to_account_info(),
                   authority: ctx.accounts.reward_token_mint.to_account_info(),
               },
               signer,
           );

           // Mint 1 token, accounting for decimals of mint
           let amount = (1u64)
               .checked_mul(10u64.pow(ctx.accounts.reward_token_mint.decimals as u32))
               .unwrap();

           mint_to(cpi_ctx, amount)?;
        } else {
            ctx.accounts.player_data.lose = ctx.accounts.player_data.lose.checked_add(1).unwrap();
            // CPI Context
            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Burn {
                    mint: ctx.accounts.reward_token_mint.to_account_info(),
                    from: ctx.accounts.player_token_account.to_account_info(),
                    authority: ctx.accounts.player.to_account_info(),
                },
            );

            // Burn 1 token, accounting for decimals of mint
            let amount = (1u64)
                .checked_mul(10u64.pow(ctx.accounts.reward_token_mint.decimals as u32))
                .unwrap();

            burn(cpi_ctx, amount)?;
        }

       Ok(())
   }

   // Mint tokens to player token account
   pub fn air_drop(ctx: Context<AirDrop>) -> Result<()> {

       // PDA seeds and bump to "sign" for CPI
       let seeds = b"reward";
       let bump = *ctx.bumps.get("reward_token_mint").unwrap();
       let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];

       // CPI Context
       let cpi_ctx = CpiContext::new_with_signer(
           ctx.accounts.token_program.to_account_info(),
           MintTo {
               mint: ctx.accounts.reward_token_mint.to_account_info(),
               to: ctx.accounts.player_token_account.to_account_info(),
               authority: ctx.accounts.reward_token_mint.to_account_info(),
           },
           signer,
       );

       // Mint 1 token, accounting for decimals of mint
       let amount = (1u64)
           .checked_mul(10u64.pow(ctx.accounts.reward_token_mint.decimals as u32))
           .unwrap();

       mint_to(cpi_ctx, amount)?;
       Ok(())
   }

}

#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(
        mut,
        address = ADMIN_PUBKEY
    )]
    pub admin: Signer<'info>,

    // The PDA is both the address of the mint account and the mint authority
    #[account(
        init,
        seeds = [b"reward"],
        bump,
        payer = admin,
        mint::decimals = 9,
        mint::authority = reward_token_mint,

    )]
    pub reward_token_mint: Account<'info, Mint>,

    ///CHECK: Using "address" constraint to validate metadata account address
    #[account(
        mut,
        address=find_metadata_account(&reward_token_mint.key()).0
    )]
    pub metadata_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitPlayer<'info> {
    #[account(
        init,
        payer = player,
        space = 8 + 8,
        seeds = [b"player", player.key().as_ref()],
        bump,
    )]
    pub player_data: Account<'info, PlayerData>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AirDrop<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        mut,
        seeds = [b"player", player.key().as_ref()],
        bump,
    )]
    pub player_data: Account<'info, PlayerData>,

    // Initialize player token account if it doesn't exist
    #[account(
        init_if_needed,
        payer = player,
        associated_token::mint = reward_token_mint,
        associated_token::authority = player
    )]
    pub player_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"reward"],
        bump,
    )]
    pub reward_token_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GuessOne<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        mut,
        seeds = [b"player", player.key().as_ref()],
        bump,
    )]
    pub player_data: Account<'info, PlayerData>,

    // Initialize player token account if it doesn't exist
    #[account(
        init_if_needed,
        payer = player,
        associated_token::mint = reward_token_mint,
        associated_token::authority = player
    )]
    pub player_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"reward"],
        bump,
    )]
    pub reward_token_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GuessTwo<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    #[account(
        mut,
        seeds = [b"player", player.key().as_ref()],
        bump,
    )]
    pub player_data: Account<'info, PlayerData>,

    // Initialize player token account if it doesn't exist
    #[account(
        init_if_needed,
        payer = player,
        associated_token::mint = reward_token_mint,
        associated_token::authority = player
    )]
    pub player_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"reward"],
        bump,
    )]
    pub reward_token_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PlayerData {
    pub win: u8,
    pub lose: u8
}

pub fn xorshift64(seed: u64) -> u64 {
    let mut x = seed;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}
