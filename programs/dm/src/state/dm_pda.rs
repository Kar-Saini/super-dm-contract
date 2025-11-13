use anchor_lang::prelude::{
    borsh::{BorshDeserialize, BorshSerialize},
    *,
};

#[account]
#[derive(InitSpace)]
pub struct DMPda {
    pub influencer_pubkey: Pubkey,
    pub sol_attached: u64,
    pub user_pubkey: Pubkey,
    pub created_at: i64,
    pub nonce: u64,
    pub dm_type: DMType,
    pub status: Status,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Copy, InitSpace)]
pub enum DMType {
    Support,
    Guidance,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Copy, InitSpace)]
pub enum Status {
    Refunded,
    Rejected,
    Pending,
    Accepted,
}
