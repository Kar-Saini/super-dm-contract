use anchor_lang::prelude::*;

use crate::state::{dm_escrow::{self, DMEscrow}, dm_pda::DMPda};

#[derive(Accounts)]
#[instruction(nonce:u64, sender_pubkey:Pubkey)]
pub struct ClaimDM<'info> {
    #[account(mut)]
    pub infleuncer: Signer<'info>,
    #[account(
        mut, 
        seeds = [
            b"dm_escrow_pda", 
            sender_pubkey.as_ref(), 
            dm_pda.key().as_ref(), 
            nonce.to_be_bytes()
            ],
        bump
    )]
    pub dm_escrow_pda : Account<'info, DMEscrow>, 
    #[account(
        mut, 
        seeds = [
            b"dm_pda", 
            sender_pubkey.as_ref(), 
            infleuncer.key().as_ref(), 
            nonce.to_be_bytes()
            ], 
        bump
    )]
    pub dm_pda: Account<'info, DMPda>,

    pub system_program :  Program<'info, System>
}


pub fn handler(
    ctx: Context<ClaimDM>,
    _nonce: u64,
    _sender_pubkey: Pubkey,
) -> Result<()> {
    let dm_escrow_pda = &mut ctx.accounts.dm_escrow_pda;
    let dm_pda = &mut ctx.accounts.dm_pda;
    let influencer = &ctx.accounts.influencer;

    require!(
        dm_pda.status == Status::Pending,
        CustomError::InvalidStatus
    );

    let seeds = &[
        b"dm_escrow_pda",
        dm_pda.sender.as_ref(),          
        dm_pda.key().as_ref(),
        dm_pda.nonce.to_be_bytes().as_ref(),
        &[dm_escrow_pda.bump],
    ];
    let signer_seeds = &[&seeds[..]];

    let amount = **dm_escrow_pda.to_account_info().lamports.borrow();
    let ix = system_program::Transfer {
        from: dm_escrow_pda.to_account_info(),
        to: influencer.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        ix,
        signer_seeds,
    );

    system_program::transfer(cpi_ctx, amount)?;
    dm_pda.status = Status::Claimed;

    Ok(())
}
