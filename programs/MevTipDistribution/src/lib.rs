use anchor_lang::prelude::*;

#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

use crate::{
    state::{ClaimStatus, TipDistributionAccount},
};

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    // Required fields
    name: "MEV Rewards Distribution Program",
    contacts: "email:team3301@sygnum.com",
    // Optional Fields
    preferred_languages: "en",
    source_code: "https://github.com/tejareddy8888/Solana-programs"
}

pub mod state;

declare_id!("4R3gSG8BpU4t19KYj8CfnbtRpnT8gtk4dvTHxVRwc2r7");

#[program]
pub mod mev_tip_distribution {
    use super::*;

    /// Claims tokens from the [TipDistributionAccount].
    pub fn claim(ctx: Context<Claim>, bump: u8, amount: u64,) -> Result<()> {
        let claim_status = &mut ctx.accounts.claim_status;
        claim_status.bump = bump;

        let claimant_account = &mut ctx.accounts.claimant;
        let tip_distribution_account = &mut ctx.accounts.tip_distribution_account;

        // Perform the claim transfer
        TipDistributionAccount::claim(
            tip_distribution_account.to_account_info(),
            claimant_account.to_account_info(),
            amount,
        )?;

        // Mark it claimed.
        claim_status.amount = amount;
        // claim_status.is_claimed = true;

        emit!(ClaimedEvent {
            tip_distribution_account: tip_distribution_account.key(),
            payer: ctx.accounts.payer.key(),
            claimant: claimant_account.key(),
            amount
        });

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(_bump: u8, _amount: u64)]
pub struct Claim<'info> {
    #[account(mut, rent_exempt = enforce)]
    pub tip_distribution_account: Account<'info, TipDistributionAccount>,

    /// Status of the claim. Used to prevent the same party from claiming multiple times.
    #[account(
        init,
        rent_exempt = enforce,
        seeds = [
            ClaimStatus::SEED,
            claimant.key().as_ref(),
            tip_distribution_account.key().as_ref()
        ],
        bump,
        space = ClaimStatus::SIZE,
        payer = payer
    )]
    
    pub claim_status: Account<'info, ClaimStatus>,

    /// CHECK: This is safe.
    /// Receiver of the funds.
    #[account(mut)]
    pub claimant: AccountInfo<'info>,

    /// Who is paying for the claim.
    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// Events
#[event]
pub struct ClaimedEvent {
    /// [TipDistributionAccount] claimed from.
    pub tip_distribution_account: Pubkey,

    /// User that paid for the claim, may or may not be the same as claimant.
    pub payer: Pubkey,

    /// Account that received the funds.
    pub claimant: Pubkey,

    /// Amount of funds to distribute.
    pub amount: u64,
}