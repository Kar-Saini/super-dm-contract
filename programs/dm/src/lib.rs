#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

declare_id!("4w1BnTYY9bHrbHVFSYF3BvHzZBtNdYqpzMK51nj61dDq");

pub mod instructions;
pub mod state;

use instructions::*;
use state::dm_pda::DMType;
#[program]
pub mod dm {

    use super::*;

    pub fn init_influencer_profile(
        ctx: Context<init_influencer_profile::InitInfluencerProfile>,
    ) -> Result<()> {
        init_influencer_profile::handler(ctx)
    }
    pub fn init_dm_pda(
        ctx: Context<init_dm_pda::InitDMPda>,
        sol_attahced: u64,
        dm_type: DMType,
        nonce: u64,
    ) -> Result<()> {
        init_dm_pda::handler(ctx, sol_attahced, dm_type, nonce)
    }

    pub fn init_dm_escrow(
        ctx: Context<init_dm_escrow::InitDMEscrow>,
        threshold: i64,
        sol_attahced: u64,
        nonce: u64,
        dm_pda_pubkey: Pubkey,
    ) -> Result<()> {
        init_dm_escrow::handler(ctx, threshold, sol_attahced, nonce, dm_pda_pubkey)
    }
}
