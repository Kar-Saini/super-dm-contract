use anchor_lang::prelude::*;

use crate::state::{dm_escrow, };
use crate::state::{dm_escrow::DMEscrow, dm_pda::DMPda};
use crate::{dm, instruction};

#[derive(Accounts)]
#[instruction(dm_pda_pubkey:Pubkey)]
pub struct InitDMEscrow<'info> {
    #[account(mut)]
    pub user : Signer<'info>, 
    #[account(
            init, 
            payer = user,
            space = 8 + DMEscrow::INIT_SPACE, 
            seeds = [b"dm_escrow_pda", user.key().as_ref(),dm_pda_pubkey.as_ref()],
            bump
        )
    ]
    pub dm_escrow_pda:Account<'info, DMEscrow>,
    pub system_program : Program<'info, System>
}

pub fn handler(ctx: Context<InitDMEscrow>, threshold: i64, sol_attached: u64, nonce: u64, dm_pda_pubkey:Pubkey)->Result<()> {

    let dm_escrow_pda = &mut ctx.accounts.dm_escrow_pda;
    dm_escrow_pda.dm_pda_pubkey = dm_pda_pubkey;
    dm_escrow_pda.created_at = Clock::get()?.unix_timestamp;
    dm_escrow_pda.expires_at = Clock::get()?.unix_timestamp + threshold;
    dm_escrow_pda.nonce = nonce;
    dm_escrow_pda.sol_attached = sol_attached;


    let ix = system_instruction::transfer(
        ctx.accounts.user.key(), 
        ctx.accounts.dm_escrow_pda.key(), 
        sol_attached*LAMPORTS_PER_SOL);

        invoke(&ix, 
            &[
            ctx.accounts.user.to_account_info(),
            ctx.accounts.dm_escrow_pda.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ]
    )?;

    Ok(())
}
