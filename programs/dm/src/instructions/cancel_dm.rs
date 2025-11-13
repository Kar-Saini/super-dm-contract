use anchor_lang::prelude::*;

use crate::state::{dm_escrow::DMEscrow, dm_pda::{DMPda, Status}};

#[derive(Accounts)]
#[instruction(dm_pda_pubkey:Pubkey, nonce:u64, influencer_pubkey:Pubkey)]
pub struct CancelDM<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut, 
        close = user,
        seeds = [b"dm_escrow_pda", user.key().as_ref(), dm_pda_pubkey.as_ref(), nonce.to_be_bytes()],
        bump
    )]
    pub dm_escrow_pda: Account<'info, DMEscrow>,
    #[account(
        mut, 
        seeds = [b"dm_pda", user.key().as_ref(), influencer_pubkey.as_ref(), nonce.to_be_bytes()], 
        bump
    )]
    pub dm_pda :Account<'info, DMPda>, 
    pub system_program : Program<'info, System>
}

pub fn handler(ctx: Context<CancelDM>, _dm_pda_pubkey: Pubkey, _nonce: u64) -> Result<()> {
    let dm_escrow_pda = &mut ctx.accounts.dm_escrow_pda;
    let dm_pda = &mut ctx.accounts.dm_pda;
    require!(dm_escrow_pda.status == Status::Pending, CustomError::InvalidStatus);
    require!(dm_escrow_pda.expires_at > Clock::get()?.unix_timestamp, CustomError::InvalidStatus);

    dm_pda.status = Status::Cancelled;
    Ok(())
}