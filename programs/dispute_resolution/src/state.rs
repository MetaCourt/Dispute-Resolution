use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey;

pub const METADATA_PROGRAM_ID: Pubkey = pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

pub const SETTINGS_PDA: &[u8] = b"settings";
pub const COURT_TREASURY_PDA: &[u8] = b"court_treasury_token_account";
pub const COURT_TREASURY_AUTHORITY_PDA_SEED: &[u8] = b"court_treasury_authority";
pub const JUROR_PDA: &[u8] = b"juror";
pub const METADATA_PDA: &[u8] = b"metadata";
pub const EDITION_PDA: &[u8] = b"edition";

pub const MAX_URI_LENGTH: usize = 200;

pub const SETTINGS_SIZE: usize = 8 + // discriminator length
32 + // master admin
32 + // admin
32 + // court token
32 + // juror NFT creator
32 + // MetaCourt treasury token account
32 + // MetaCourt treasury token account authority
1 + // MetaCourt treasury token account authority bump
8; // dispute raising fee

#[account]
#[derive(Default)]
pub struct Settings {
    pub master_admin: Pubkey,
    pub admin: Pubkey,
    pub court_token: Pubkey,
    pub juror_creator: Pubkey,
    pub court_treasury_token_account: Pubkey,
    pub court_treasury_token_account_authority: Pubkey,
    pub court_treasury_token_account_bump: u8,
    pub raise_dispute_fee: u64, // TODO how to get additional fee for the reward?
}

pub const DISPUTE_SIZE: usize = 8 + // discriminator length
8 + // dispute value
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

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct DisputeType {
    pub dispute_value: u64,
    pub required_stake_amount: u64,
    pub join_juror_deadline: i64,
    pub draw_juror_deadline: i64,
    pub closure_deadline: i64,
    pub applicants: Vec<PartyType>,
    pub respondents: Vec<PartyType>,
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

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct PartyType {
    pub address: Pubkey,
    pub share: u8, // In percentage
    pub evidence_uri: String,
    pub fingerprint: [u8; 32], // hash of the evidence
}

pub const JUROR_RESERVATION_ENTRY_SIZE: usize = 8 + // discriminator length
32 +
32;

#[account]
#[derive(Default)]
pub struct JurorReservationEntry {
    pub address: Pubkey,
    pub dispute: Pubkey,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum DisputeStatus {
    Initialized = 0,
    Approved = 1,
    Started = 2,
}

impl Default for DisputeStatus {
    fn default() -> Self {
        DisputeStatus::Initialized
    }
}

#[derive(Copy, Clone, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum JurorOpinion {
    None = 0,
    Applicant = 1,
    Respondent = 2,
}

impl Default for JurorOpinion {
    fn default() -> Self {
        JurorOpinion::None
    }
}
