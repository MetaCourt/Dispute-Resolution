use anchor_lang::prelude::*;

#[error_code]
pub enum CourtError {
    #[msg("You are not authorize to join this dispute.")]
    NoJoinAuthorize,
    #[msg("Metadata of the juror NFT is not initialized.")]
    MetadataNotInitialized,
    #[msg("Master Edition of the juror NFT is not initialized.")]
    MasterEditionNotInitialized,
    #[msg("Juror NFT is not valid.")]
    NFTNotValid,
    #[msg("Total shares can not exceed more than 100 percent.")]
    SharesExceeded,
    #[msg("First applicant party should be the transaction fee payer.")]
    PayerNotMatchFirstApplicantParty,
    #[msg("Length of provided jurors is not correct.")]
    JurorNumbersNotCorrect,
}
