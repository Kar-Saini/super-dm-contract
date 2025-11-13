use anchor_lang::InitSpace;

#[account]
#[derive(InitSpace)]
pub struct DMEscrow {
    pub sol_attached: u64,
    pub influencer_pubkey: Pubkey,
    pub dm_pda_pubkey: Pubkey,
    pub sender_pubkey: Pubkey,
    pub nonce: u64,
}
