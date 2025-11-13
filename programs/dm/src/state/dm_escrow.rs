use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct DMEscrow {
    pub sol_attached: u64,
    pub dm_pda_pubkey: Pubkey,
    pub created_at: i64,
    pub expires_at: i64,
    pub nonce: u64,
}
