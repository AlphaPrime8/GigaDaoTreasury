use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod gdmultisig {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}


// structs
#[account]
#[derive(Default)]
pub struct Market {
    pub mint: Pubkey,
    pub balances: Pubkey,
    pub wsol_vault: Pubkey,
    pub lot_vault: Pubkey,
    pub asks: Pubkey,
    pub bids: Pubkey,
}



#[account]
#[derive(Default)]
pub struct AuthAccount {}


#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone)]
pub struct Withdrawal {
    pub criteria: String,
    pub hunter: Pubkey,
    pub acceptor: Pubkey,
    pub amount: u64,
}



#[error_code]
pub enum ErrorCode {
    #[msg("Generic Error")]
    GenericProgramError,
}

