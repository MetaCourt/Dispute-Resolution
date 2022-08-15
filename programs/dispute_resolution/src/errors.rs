use anchor_lang::prelude::*;

#[error_code]
pub enum CourtError {
    #[msg("You are not authorize to join this dispute.")]
    NoJoinAuthorize,
}
