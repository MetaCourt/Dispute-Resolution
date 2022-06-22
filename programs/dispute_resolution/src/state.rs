use anchor_lang::prelude::*;

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
    pub status: DisputeStatus,
    pub applicants: Vec<Party>,
    pub respondents: Vec<Party>,
}

#[derive(Clone, Default, AnchorSerialize, AnchorDeserialize)]
pub struct Party {
    pub address: Pubkey,
    pub joined: bool,
    pub share: u8, // In percentage
    pub uri: String,
    pub fingerprint: [u8; 32],
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub enum DisputeStatus {
    Initialized,
    Accepted,
    ExtraTime,
    Finished,
}

impl Default for DisputeStatus {
    fn default() -> Self {
        DisputeStatus::Initialized
    }
}
