use anchor_lang::prelude::*;

declare_id!("FL4xnJd2Nzea9pqg4hEzkFVYaZP5h616qQS5cSrtGArA");

#[program]
pub mod non_custodial_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
