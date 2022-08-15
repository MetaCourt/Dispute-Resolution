use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey;

pub const COURT_TOKEN: Pubkey = pubkey!("CotjBMa7GVLUP6ajjDbCxoNZBAu9zfkLZzcU5wCLC2Hx");
pub const ADMIN: Pubkey = pubkey!("CotjBMa7GVLUP6ajjDbCxoNZBAu9zfkLZzcU5wCLC2Hx");
pub const COURT_TREASURY_TOKEN_ACCOUNT: Pubkey =
    pubkey!("7ts4zibEhu1rL6CBXyGopZ7PFRLL8gPFKkdBQNAQCM6W");
pub const RAISE_DISPUTE_FEE: u64 = 1; // TODO fee to create dispute

pub const DISPUTE_SIZE: usize = 32 + // maker
1; // dispute status
pub const PARTY_SIZE: usize = 32 + // address
1 + // joined
1 + // share (percent)
4 + MAX_URI_LENGTH +
32; // fingerprint hash

pub const MAX_URI_LENGTH: usize = 200;

#[account]
#[derive(Default)]
pub struct Dispute {
    pub dispute_value: u64,
    pub dispute_closure_timestamp: i64,
    pub status: DisputeStatus,
    pub applicants: Vec<Party>,
    pub respondents: Vec<Party>,
}

#[derive(Clone, Default, AnchorSerialize, AnchorDeserialize)]
pub struct Party {
    pub address: Pubkey,
    pub joined: bool,
    pub share: u8, // In percentage
    pub evidence_uri: String,
    pub fingerprint: [u8; 32], // hash of the evidence
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub enum DisputeStatus {
    Initialized,
    Approved,
    ExtraTime,
    Finished,
}

impl Default for DisputeStatus {
    fn default() -> Self {
        DisputeStatus::Initialized
    }
}
