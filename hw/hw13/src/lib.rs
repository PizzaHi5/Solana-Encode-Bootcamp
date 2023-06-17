// The program I wrote for this homework
use anchor_lang::prelude::*;

declare_id!("GyxSxejrutiCUwT1hfGMmTMYtRNUwvvMAB18nYBCcvgo");

#[program]
pub mod hw13 {
    use super::*;

    pub fn initialize(ctx: Context<SetData>) -> Result<()> {
        ctx.accounts.data_account.data = 100 as u64;
        Ok(())
    }

    // Trying to write a public getter function
    pub fn get_data(ctx: Context<SetData>) -> Result<u64> {
        Ok(ctx.accounts.data_account.data)
    }
}

#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub data_account: Account<'info, Starting>
}

#[account]
#[derive(Default)]
pub struct Starting {
    pub data: u64,
}