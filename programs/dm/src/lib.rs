use anchor_lang::prelude::*;

declare_id!("4w1BnTYY9bHrbHVFSYF3BvHzZBtNdYqpzMK51nj61dDq");

pub mod state;

#[program]
pub mod dm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
