use anchor_lang::prelude::*;

declare_id!("AuWmcCynVKE4f1mCZeBBjJczouC6SmZEUnJk8SGaewyB");

#[program]
pub mod self_custodial_facebook {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
