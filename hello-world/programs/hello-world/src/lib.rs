use anchor_lang::prelude::*;

declare_id!("CHtBZwA9gJFcFziPBsaJP2NzRPGqqtA8BUbB4e3N6JKs");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
