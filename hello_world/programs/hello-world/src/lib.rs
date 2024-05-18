use anchor_lang::prelude::*;

declare_id!("Cc8JgwAKo5bGSuDRkzaZVnjssohVW8rcRKXrwR1eohVD");

#[program]
pub mod hello_world {
    use super::*;

    pub fn hello_world(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello world, from solana smart contract");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
