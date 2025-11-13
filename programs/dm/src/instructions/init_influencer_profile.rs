
use anchor_lang::prelude::*;
use crate::state::{};


#[derive(Accounts)]
pub struct InitInfluencerProfile<'info> {
    #[account(mut)]
    pub user: Signer<'inof>,
    #[account(
        space = 8 + InfluencerProfile::INIT_SPACE, 
        payer  = user, 
        seeds = [b"influencer", user.key().as_ref()],
        bump
    )]
    pub influencer_profile:Account<'info,InfluencerProfile >, 
    pub system_program : System<'indo, System>
}
