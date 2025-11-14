use anchor_lang::prelude::*;

use crate::state::{
    dm_escrow::DMEscrow,
    dm_pda::{DMPda, Status},
    influencer_profile::InfluencerProfile,
};

#[derive(Accounts)]
#[instruction(nonce: u64)]
pub struct ClaimExpiredDM<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub influencer: SystemAccount<'info>,

    #[account(
        mut,
        close = user,
        seeds = [
            b"dm_escrow_pda",
            user.key().as_ref(),
            dm_pda.key().as_ref(),
            &nonce.to_be_bytes(),
        ],
        bump
    )]
    pub dm_escrow_pda: Account<'info, DMEscrow>,

    #[account(
        mut,
        seeds = [
            b"dm_pda",
            user.key().as_ref(),
            influencer.key().as_ref(),
            &nonce.to_be_bytes(),
        ],
        bump
    )]
    pub dm_pda: Account<'info, DMPda>,
}

pub fn handler(ctx: Context<ClaimExpiredDM>, _sender_pubkey: Pubkey, nonce: u64) -> Result<()> {
    let dm_pda = &mut ctx.accounts.dm_pda;
    let dm_escrow_pda = &mut ctx.accounts.dm_escrow_pda;

    require!(dm_pda.status == Status::Pending, CustomError::InvalidStatus);

    require!(
        dm_escrow_pda.expires_at < Clock::get()?.unix_timestamp,
        CustomError::NotExpired
    );

    dm_pda.status = Status::Refunded;

    Ok(())
}
