use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::{dm, state::{dm_pda::{DMPda, DMType, Status}, influencer_profile::InfluencerProfile}};

#[derive(Accounts)]
#[instruction(influencer_pubkey:Pubkey, nonce:u64)]
pub struct InitDMPda<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user, 
        space = 8 + DMPda::INIT_SPACE, 
        seeds = [b"dm_pda", user.key().as_ref(), influencer_pubkey.as_ref(), nonce.to_be_bytes()], 
        bump
    )]
    pub dm_pda :Account<'info, DMPda>, 
    #[account(
        mut,
        seeds = [b"influencer",influencer_pubkey.as_ref()], 
        bump
    )]
    pub infleuncer_pda:Account<'info, InfluencerProfile>,
    pub system_program:Program<'info, System>
}


pub fn handler(ctx:Context<InitDMPda>, sol_attached:u64, dm_type:DMType, nonce:u64 )->Result<()>{

    let dm_pda = &mut ctx.accounts.dm_pda;
    dm_pda.created_at = Clock::get()?.unix_timestamp;
    dm_pda.sol_attached = sol_attached;
    dm_pda.influencer_pubkey = ctx.accounts.infleuncer_pda.public_key;
    dm_pda.dm_type = dm_type;
    dm_pda.status = Status::Pending;
    dm_pda.nonce = nonce;
    dm_pda.user_pubkey = ctx.accounts.user.key();


    if  matches!(dm_type, DMType::Support) {
        let ix = system_instruction::transfer(
            ctx.accounts.user.key(), 
            ctx.accounts.infleuncer_pda.public_key,    
            sol_attached* LAMPORTS_PER_SOL);

            invoke(&ix, &[
            ctx.accounts.user.to_account_info(), 
            ctx.accounts.infleuncer_pda.to_account_info(), 
            ctx.accounts.system_program.to_account_info(), 

        ])?;
    }

    Ok(())
}