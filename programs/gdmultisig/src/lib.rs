use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("2omcykYnUGQW8tDGKZFMuJHAswrfMDAgMTkBo3Kd6Woj");

// consts
pub const MAX_WITHDRAW_VALUE_MONTHLY_USD: f64 = 25_000.;
pub const MAX_WITHDRAWS_MONTHLY: usize = 100;
const TREASURY_AUTH_PDA_SEED: &[u8] = b"treasury_auth_pda_seed";
pub const TREASURY_ACCOUNT_LEN: usize = 2048;
pub const MIN_ACCOUNT_LEN: usize = 9;
pub const SECONDS_PER_MONTH: u64 = 2_592_000;
// pub const USDC_DECIMALS: u64 = 6;
// pub const WSOL_DECIMALS: u64 = 9;

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

     pub fn execute_withdrawal(
         ctx: Context<ExecuteWithdrawal>,
         amount: u64,
         withdraw_usdc: bool,
     ) -> Result<()> {

        // check that signer is a councillor
        let _owner_index = ctx.accounts.treasury.councillors
            .iter()
            .position(|a| a == ctx.accounts.signer.key)
            .ok_or(ErrorCode::InvalidCouncillor)?;

         let current_time = Clock::get().unwrap().unix_timestamp as u64;
         let one_month_ago_seconds = current_time - SECONDS_PER_MONTH;

         // check value withdrawn in last month
         let mut num_withdrawals_in_last_month = 0;
         let mut usd_value_withdrawn_in_last_month: f64 = 0.;
         for i in 0..ctx.accounts.treasury.withdrawals.len() {
             let withdrawal = &ctx.accounts.treasury.withdrawals[i];
             if withdrawal.timestamp > one_month_ago_seconds {
                 num_withdrawals_in_last_month += 1;
                 usd_value_withdrawn_in_last_month += withdrawal.usd_value;
             }
         }
         // compute usd amount eligible to withdraw
         if num_withdrawals_in_last_month >= MAX_WITHDRAWS_MONTHLY {
             return err!(ErrorCode::TooManyWithdraws);
         }
         if usd_value_withdrawn_in_last_month >= MAX_WITHDRAW_VALUE_MONTHLY_USD {
             return err!(ErrorCode::ExceededWithdrawLimit);
         }
         let usd_value_eligible_to_withdraw = MAX_WITHDRAW_VALUE_MONTHLY_USD - usd_value_withdrawn_in_last_month;
         if amount as f64 > usd_value_eligible_to_withdraw {
             return err!(ErrorCode::ExceededWithdrawLimit);
         }

         // execute withdraw
         let treasury_address = ctx.accounts.treasury.key();
         let (treasury_auth_pda, bump_seed) = Pubkey::find_program_address(&[treasury_address.as_ref(), TREASURY_AUTH_PDA_SEED], ctx.program_id);
         if treasury_auth_pda != ctx.accounts.treasury_auth_pda.key() {
             return err!(ErrorCode::InvalidAuthPda);
         }
         let seeds = &[treasury_address.as_ref(), &TREASURY_AUTH_PDA_SEED[..], &[bump_seed]];
         let signer = &[&seeds[..]];

         let amount_withdrawn;
         if withdraw_usdc {
             let cpi_accounts = Transfer {
                 from: ctx.accounts.usdc_vault.to_account_info(),
                 to: ctx.accounts.receiver_usdc_ata.to_account_info(),
                 authority: ctx.accounts.treasury_auth_pda.to_account_info(),
             };
             let cpi_program = ctx.accounts.token_program.to_account_info();
             let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
             let amount_usdc_lots = amount * 1_000_000; // usdc has 6 decimals
             token::transfer(cpi_ctx, amount_usdc_lots)?; //
             amount_withdrawn = amount;
             msg!("withdrew usdc val usd: {:?}", amount_withdrawn);
         }
         else {
             // withdraw wsol
              let cpi_accounts = Transfer {
                 from: ctx.accounts.wsol_vault.to_account_info(),
                 to: ctx.accounts.receiver_wsol_ata.to_account_info(),
                 authority: ctx.accounts.treasury_auth_pda.to_account_info(),
             };
             let cpi_program = ctx.accounts.token_program.to_account_info();
             let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

             // convert at rate of $10/SOL
             let amount_sol_to_withdraw = amount as f64 / 10.;
             msg!("amount sol withdraw: {:?}", amount_sol_to_withdraw);
             let amount_lamports_to_withdraw = (amount_sol_to_withdraw * 1_000_000_000.) as u64; // wsol has 9 decimals
             msg!("amount lams withdraw: {:?}", amount_lamports_to_withdraw);

             token::transfer(cpi_ctx, amount_lamports_to_withdraw)?; //
             amount_withdrawn = amount_lamports_to_withdraw / 1_000_000_000 * 10;
             msg!("withdrew sol val usd: {:?}", amount_withdrawn);
         }

         // record withdraw
         let new_withdrawal = Withdrawal {
             timestamp: current_time,
             usd_value: amount_withdrawn as f64,
         };
         let withdraws = &mut ctx.accounts.treasury.withdrawals;
         if withdraws.len() >= MAX_WITHDRAWS_MONTHLY {
             withdraws.pop();
         }
         withdraws.push(new_withdrawal);
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
    pub receiver_wsol_ata: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub receiver_usdc_ata: Box<Account<'info, TokenAccount>>,
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
    pub usd_value: f64,
}

#[account]
#[derive(Default)]
pub struct AuthAccount {}

#[error_code]
pub enum ErrorCode {
    #[msg("Generic Error")]
    GenericProgramError,
    #[msg("InvalidCouncillor")]
    InvalidCouncillor,
    #[msg("Too Many Withdraws")]
    TooManyWithdraws,
    #[msg("Exceeded Withdraw Limit")]
    ExceededWithdrawLimit,
    #[msg("Invalid Authorizer PDA.")]
    InvalidAuthPda,
}

