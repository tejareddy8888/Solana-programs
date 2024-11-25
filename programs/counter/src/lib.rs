use anchor_lang::prelude::*;

declare_id!("G9pg6GKNKCMFZXZZe99daoSXeJQdGfWerLgBnUxv8XLX");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("ProgramId: {:?}", ctx.program_id);
        msg!("accounts: {:?}", ctx.accounts);
        msg!("remaining_accounts: {:?}", ctx.remaining_accounts);
        Ok(())
    }

    pub fn initialize_counter(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Initialized new count. Current value: {}!", counter.count);
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        msg!("New Count Value {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts,Debug)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts,Debug)]
pub struct Increment<'info> {
    #[account(mut, signer)]
    pub counter: Account<'info, Counter>,
}

#[account]
#[derive(Debug)]
pub struct Counter {
    count: u64,
    #[account(signer)]
    authority: Account
}