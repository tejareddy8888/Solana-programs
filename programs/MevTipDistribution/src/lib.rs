use anchor_lang::prelude::*;
use std::mem::size_of;

const HEADER_SIZE: usize = 8;

#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    // Required fields
    name: "MEV Tip Distribution Program",
    project_url: "https://simpleweb3.ch/",
    policy: "https://github.com/tejareddy8888/Solana-programs"
}


declare_id!("96Ct8jQauoJNxwaR9t5MwadpNCTa718Q2LA8kASXmN9R");

#[program]
pub mod mev_tip_distribution {
    use super::*;

    /// Initialize a singleton instance of the [Config] account.
    pub fn initialize(
        ctx: Context<Initialize>,
        distribution_authority: Pubkey,
        max_tip_amount: u64,
        bump: u8,
    ) -> Result<()> {
        let cfg = &mut ctx.accounts.config;

        cfg.distribution_authority = distribution_authority;
        cfg.max_tip_amount = max_tip_amount;
        cfg.claim_counter = 0;
        cfg.bump = bump;

        cfg.validate()?;

        emit!(TipDistributionAccountInitializedEvent {
            tip_distribution_account: ctx.accounts.config.key(),
        });

        Ok(())
    }

    /// Claims tokens from the [TipDistributionAccount].
    pub fn claim(ctx: Context<Claim>, amount: u64) -> Result<()> {
        Claim::auth(&ctx)?;

        let config = &ctx.accounts.config;
        let claimant = &ctx.accounts.claimant;

        // Ensure the amount does not exceed the max_tip_amount
        require!(amount <= config.max_tip_amount, ErrorCode::ArithmeticError);

        // Perform the claim transfer
        DistributionConfig::claim(config.to_account_info(), claimant.to_account_info(), amount)?;

        msg!("Tipping amount {} to: {}!", amount, claimant.key());

        emit!(ClaimedEvent {
            tip_distribution_account: config.key(),
            claimant: claimant.key(),
            amount
        });

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [DistributionConfig::SEED],
        bump,
        payer = initializer,
        space = DistributionConfig::SIZE,
        rent_exempt = enforce
    )]
    pub config: Account<'info, DistributionConfig>,

    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub initializer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut, has_one = distribution_authority)]
    pub config: Account<'info, DistributionConfig>,

    /// CHECK: This is safe because we are only transferring lamports to this account.
    #[account(mut)]
    pub claimant: AccountInfo<'info>,

    /// The distribution authority must match the authority in the config.
    #[account(signer)]
    pub distribution_authority: Signer<'info>,

    /// Who is paying for the claim.
    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
impl Claim<'_> {
    fn auth(ctx: &Context<Claim>) -> Result<()> {
        if ctx.accounts.config.distribution_authority.key()
            != ctx.accounts.distribution_authority.key()
        {
            Err(ErrorCode::Unauthorized.into())
        } else {
            Ok(())
        }
    }
}

#[account]
#[derive(Default)]
pub struct DistributionConfig {
    /// Account with authority over this PDA to distribute.
    pub distribution_authority: Pubkey,

    /// The max lamports that can be transferred as part of this program in a single transaction.
    pub max_tip_amount: u64,

    pub claim_counter: u8,

    /// The bump used to generate this account.
    pub bump: u8,
}

impl DistributionConfig {
    pub const SEED: &'static [u8] = b"DISTRIBUTION_CONFIG_ACCOUNT";
    pub const SIZE: usize = HEADER_SIZE + size_of::<Self>();

    pub fn validate(&self) -> Result<()> {
        require!(self.max_tip_amount > 0, ErrorCode::ArithmeticError);
        require!(
            self.distribution_authority != Pubkey::default(),
            ErrorCode::AccountValidationFailure
        );
        Ok(())
    }

    pub fn claim(from: AccountInfo, to: AccountInfo, amount: u64) -> Result<()> {
        msg!("Transferring from {} to {} amount: {}!", from.key, to.key, amount);
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

// Events
#[event]
pub struct ClaimedEvent {
    /// [TipDistributionAccount] claimed from.
    pub tip_distribution_account: Pubkey,

    /// Account that received the funds.
    pub claimant: Pubkey,

    /// Amount of funds to distribute.
    pub amount: u64,
}

#[event]
pub struct TipDistributionAccountInitializedEvent {
    pub tip_distribution_account: Pubkey,
}

// Error Codes
#[error_code]
pub enum ErrorCode {
    #[msg("Account failed validation.")]
    AccountValidationFailure,

    #[msg("Encountered an arithmetic under/overflow error.")]
    ArithmeticError,

    #[msg("Unauthorized operation.")]
    Unauthorized,
}