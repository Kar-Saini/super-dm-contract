use anchor_lang::prelude::*;

#[deriver(Accounts)]
pub struct InitDMPda<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        space = 8 + DMPda::INIT_SPACE, 
        payer = user, 
        seeds = [b"dm_pda", user.key().as_ref()]
    )]
}
