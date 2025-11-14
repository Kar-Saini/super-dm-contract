use anchor_lang::prelude::program::invoke_signed;
use anchor_lang::prelude::*;
use solana_program::system_instruction;

use crate::state::{
    dm_escrow::DMEscrow,
    dm_pda::{DMPda, Status},
};

#[derive(Accounts)]
#[instruction(nonce: u64, sender_pubkey: Pubkey)]
pub struct ClaimDM<'info> {
    #[account(mut)]
    pub influencer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"dm_escrow_pda",
            sender_pubkey.as_ref(),
            dm_pda.key().as_ref(),
            nonce.to_be_bytes(),
        ],
        bump
    )]
    pub dm_escrow_pda: Account<'info, DMEscrow>,

    #[account(
        mut,
        seeds = [
            b"dm_pda",
            sender_pubkey.as_ref(),
            influencer.key().as_ref(),
            nonce.to_be_bytes(),
        ],
        bump
    )]
    pub dm_pda: Account<'info, DMPda>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ClaimDM>, nonce: u64, sender_pubkey: Pubkey) -> Result<()> {
    let dm_escrow_pda = &ctx.accounts.dm_escrow_pda;
    let dm_pda = &mut ctx.accounts.dm_pda;
    let influencer = &ctx.accounts.influencer;

    require!(dm_pda.status == Status::Pending, CustomError::InvalidStatus);

    let amount = **dm_escrow_pda.to_account_info().lamports.borrow();

    let escrow_seeds: &[&[u8]] = &[
        b"dm_escrow_pda",
        sender_pubkey.as_ref(),
        dm_pda.key().as_ref(),
        &nonce.to_be_bytes(),
        &[dm_escrow_pda.bump],
    ];
    let signer_seeds = &[escrow_seeds];

    let ix = system_instruction::transfer(&dm_escrow_pda.key(), &influencer.key(), amount);

    invoke_signed(
        &ix,
        &[
            dm_escrow_pda.to_account_info(),
            influencer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        signer_seeds,
    )?;

    dm_pda.status = Status::Accepted;

    Ok(())
}
