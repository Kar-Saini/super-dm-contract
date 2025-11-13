use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct InfluencerProfile {
    pub public_key: Pubkey,
}
