use std::mem::size_of;

use anchor_lang::prelude::*;

const HEADER_SIZE: usize = 8;

#[error_code]
pub enum ErrorCode {
    #[msg("Account failed validation.")]
    AccountValidationFailure,

    #[msg("Encountered an arithmetic under/overflow error.")]
    ArithmeticError,

    #[msg("Unauthorized operation")]
    Unauthorized,
}

#[account]
#[derive(Default)]
pub struct ClaimStatus {
    /// If true, the tokens have been claimed.
    pub is_claimed: bool,

    /// Authority that claimed the tokens. Allows for delegated rewards claiming.
    pub claimant: Pubkey,

    /// The payer who created the claim.
    pub claim_status_payer: Pubkey,

    /// Amount of funds claimed.
    pub amount: u64,

    /// The bump used to generate this account
    pub bump: u8,
}

#[account]
#[derive(Default)]
pub struct TipDistributionAccount {
    /// The validator's vote account, also the recipient of remaining lamports after
    /// upon closing this account.
    pub validator_vote_account: Pubkey,

    /// Epoch for which this account was created.  
    pub epoch_created_at: u64,

    /// The commission basis points this validator charges.
    pub validator_commission_bps: u16,

    /// The epoch (upto and including) that tip funds can be claimed.
    pub expires_at: u64,

    /// The bump used to generate this account
    pub bump: u8,
}

impl TipDistributionAccount {
    pub const SEED: &'static [u8] = b"TIP_DISTRIBUTION_ACCOUNT";

    pub const SIZE: usize = HEADER_SIZE + size_of::<Self>();

    pub fn validate(&self) -> Result<()> {
        Ok(())
    }

    pub fn claim_expired(from: AccountInfo, to: AccountInfo) -> Result<u64> {
        let rent = Rent::get()?;
        let min_rent_lamports = rent.minimum_balance(from.data_len());

        let amount = from
            .lamports()
            .checked_sub(min_rent_lamports)
            .ok_or(ErrorCode::ArithmeticError)?;
        Self::transfer_lamports(from, to, amount)?;

        Ok(amount)
    }

    pub fn claim(from: AccountInfo, to: AccountInfo, amount: u64) -> Result<()> {
        Self::transfer_lamports(from, to, amount)
    }

    fn transfer_lamports(from: AccountInfo, to: AccountInfo, amount: u64) -> Result<()> {
        // debit lamports
        **from.try_borrow_mut_lamports()? = from
            .lamports()
            .checked_sub(amount)
            .ok_or(ErrorCode::ArithmeticError)?;
        // credit lamports
        **to.try_borrow_mut_lamports()? = to
            .lamports()
            .checked_add(amount)
            .ok_or(ErrorCode::ArithmeticError)?;

        Ok(())
    }
}

impl ClaimStatus {
    pub const SEED: &'static [u8] = b"CLAIM_STATUS";

    pub const SIZE: usize = HEADER_SIZE + size_of::<Self>();
}
