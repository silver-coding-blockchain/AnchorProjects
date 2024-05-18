use anchor_lang::prelude::*;

declare_id!("TRPudBx5gYh9g58oWaC8sMoVPTEoE769YAS5oiz3QSa");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
