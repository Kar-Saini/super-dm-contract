use anchor_lang::InitSpace;

#[account]
#[derive(InitSpace)]
pub struct DMPda {
    pub sol_attached: u64,
    pub influencer_pubkey: Pubkey,
    pub created_at: u64,
    pub expires_at: u64,
    pub sender_pubkey: Pubkey,
    pub nonce: u64,
}
