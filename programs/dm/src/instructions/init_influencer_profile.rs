
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use crate::state::influencer_profile::InfluencerProfile;


#[derive(Accounts)]
pub struct InitInfluencerProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init, 
        space = 8 + InfluencerProfile::INIT_SPACE, 
        payer  = user, 
        seeds = [b"influencer", user.key().as_ref()],
        bump
    )]
    pub influencer_profile:Account<'info,InfluencerProfile >, 
    pub system_program : Program<'info, System>
}


pub fn handler(ctx:Context<InitInfluencerProfile>)->Result<()>{
    ctx.accounts.influencer_profile.public_key = ctx.accounts.user.key();
    Ok(())
}