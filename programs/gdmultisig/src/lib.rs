use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, sync_native, SyncNative};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// consts
pub const MAX_WITHDRAW_VALUE_MONTHLY_USD: u64 = 25_000;
pub const MAX_WITHDRAWS_MONTHLY: usize = 100;
const TREASURY_AUTH_PDA_SEED: &[u8] = b"treasury_auth_pda_seed";
pub const TREASURY_ACCOUNT_LEN: usize = 2048; // TODO do this math
pub const MIN_ACCOUNT_LEN: usize = 9;

#[program]
pub mod gdmultisig {
    use super::*;

    pub fn initialize_treasury(
        ctx: Context<InitializeTreasury>,
        councillors: Vec<Pubkey>
    ) -> Result<()> {

        let treasury = &mut ctx.accounts.treasury;
        let withdrawal_vec: Vec<Withdrawal> = Vec::with_capacity(MAX_WITHDRAWS_MONTHLY);

        treasury.councillors = councillors;
        treasury.withdrawals = withdrawal_vec;
        treasury.gigs_vault = ctx.accounts.gigs_vault.key();
        treasury.wsol_vault = ctx.accounts.wsol_vault.key();
        treasury.usdc_vault = ctx.accounts.usdc_vault.key();

        Ok(())
    }

     pub fn execute_withdrawal(ctx: Context<ExecuteWithdrawal>) -> Result<()> {







        Ok(())
    }

}

#[derive(Accounts)]
pub struct InitializeTreasury<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    init,
    payer = signer,
    space = TREASURY_ACCOUNT_LEN,
    )]
    pub treasury: Box<Account<'info, Treasury>>,
    #[account(
    init,
    seeds = [treasury.key().as_ref(), TREASURY_AUTH_PDA_SEED],
    bump,
    payer = signer,
    space = MIN_ACCOUNT_LEN)]
    pub treasury_auth_pda: Box<Account<'info, AuthAccount>>,
    pub wsol_mint: Box<Account<'info, Mint>>,
    #[account(
    init,
    token::mint = wsol_mint,
    token::authority = treasury_auth_pda,
    payer = signer)]
    pub wsol_vault: Box<Account<'info, TokenAccount>>,
    pub usdc_mint: Box<Account<'info, Mint>>,
    #[account(
    init,
    token::mint = usdc_mint,
    token::authority = treasury_auth_pda,
    payer = signer)]
    pub usdc_vault: Box<Account<'info, TokenAccount>>,
    pub gigs_mint: Box<Account<'info, Mint>>,
    #[account(
    init,
    token::mint = gigs_mint,
    token::authority = treasury_auth_pda,
    payer = signer)]
    pub gigs_vault: Box<Account<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ExecuteWithdrawal<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    mut,
    constraint = treasury.wsol_vault == wsol_vault.key(),
    constraint = treasury.usdc_vault == usdc_vault.key(),
    constraint = treasury.gigs_vault == gigs_vault.key(),
    )]
    pub treasury: Box<Account<'info, Treasury>>,
    #[account(
    mut,
    seeds = [treasury.key().as_ref(), TREASURY_AUTH_PDA_SEED],
    bump)]
    pub treasury_auth_pda: Box<Account<'info, AuthAccount>>,
    #[account(mut)]
    pub wsol_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub usdc_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub gigs_vault: Box<Account<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

// structs
#[account]
#[derive(Default)]
pub struct Treasury {
    pub councillors: Vec<Pubkey>,
    pub withdrawals: Vec<Withdrawal>,
    pub wsol_vault: Pubkey,
    pub usdc_vault: Pubkey,
    pub gigs_vault: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone)]
pub struct Withdrawal {
    pub timestamp: u64,
    pub usd_value: u64,
}

#[account]
#[derive(Default)]
pub struct AuthAccount {}

#[error_code]
pub enum ErrorCode {
    #[msg("Generic Error")]
    GenericProgramError,
}

