use anchor_lang::prelude::*;

declare_id!("2zDXpazUQnnm1R7FXkt61TSJ54nhAjTBDf5FxZvkJmYs");

#[program]
pub mod onchain_voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
