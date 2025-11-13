use anchor_lang::prelude::*;

#[derive(InitSpace)]
pub struct InfluencerProfile {
    pubkey: Pubkey,
}
