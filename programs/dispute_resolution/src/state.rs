use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey;

pub const COURT_TOKEN: Pubkey = pubkey!("CotjBMa7GVLUP6ajjDbCxoNZBAu9zfkLZzcU5wCLC2Hx");
pub const METADATA_PROGRAM_ID: Pubkey = pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
pub const JUROR_CREATOR: Pubkey = pubkey!("CotjBMa7GVLUP6ajjDbCxoNZBAu9zfkLZzcU5wCLC2Hx");
pub const ADMIN: Pubkey = pubkey!("CotjBMa7GVLUP6ajjDbCxoNZBAu9zfkLZzcU5wCLC2Hx");
pub const COURT_TREASURY_TOKEN_ACCOUNT: Pubkey =
    pubkey!("7ts4zibEhu1rL6CBXyGopZ7PFRLL8gPFKkdBQNAQCM6W");
pub const RAISE_DISPUTE_FEE: u64 = 1; // TODO fee to create dispute

pub const MAX_URI_LENGTH: usize = 200;

pub const DISPUTE_SIZE: usize = 8 + // dispute value
8 + // required stake amount
8 + // init timestamp
8 + // join juror deadline timestamp
8 + // draw juror deadline timestamp
8 + // closure deadline timestamp
2 + // dispute ready jurors
1 + // dispute status
4 + // applicants Vec prefix (parties will be added at runtime)
4 + // respondents Vec prefix (parties will be added at runtime)
(7 * JUROR_SIZE); // jurors

#[account]
#[derive(Default)]
pub struct Dispute {
    pub dispute_value: u64,
    pub required_stake_amount: u64,
    pub init_time: i64,
    pub join_juror_deadline: i64,
    pub draw_juror_deadline: i64,
    pub closure_deadline: i64,
    pub ready_jurors: u16,
    pub status: DisputeStatus,
    pub applicants: Vec<Party>,
    pub respondents: Vec<Party>,
    pub jurors: [Juror; 7],
}

pub const JUROR_SIZE: usize = 32 + // address
1 + // opinion
1; // claimed reward

#[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
pub struct Juror {
    pub address: Pubkey,
    pub opinion: JurorOpinion,
    pub claimed_reward: bool,
}

pub const PARTY_SIZE: usize = 32 + // address
1 + // joined
1 + // share (percent)
4 + MAX_URI_LENGTH +
32; // fingerprint hash

#[derive(Clone, Default, AnchorSerialize, AnchorDeserialize)]
pub struct Party {
    pub address: Pubkey,
    pub joined: bool,
    pub share: u8, // In percentage
    pub evidence_uri: String,
    pub fingerprint: [u8; 32], // hash of the evidence
}

pub const JUROR_RESERVATION_ENTRY_SIZE: usize = 32;

#[account]
#[derive(Default)]
pub struct JurorReservationEntry {
    pub address: Pubkey,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum DisputeStatus {
    Initialized,
    Approved,
    Started,
}

impl Default for DisputeStatus {
    fn default() -> Self {
        DisputeStatus::Initialized
    }
}

#[derive(Copy, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum JurorOpinion {
    Applicant,
    Respondent,
    None,
}

impl Default for JurorOpinion {
    fn default() -> Self {
        JurorOpinion::None
    }
}
