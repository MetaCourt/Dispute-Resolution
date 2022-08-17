use anchor_lang::prelude::*;

#[error_code]
pub enum CourtError {
    #[msg("Settings have been already created.")]
    SettingsAlreadyCreated,
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
    #[msg("Juror did not signed the transaction.")]
    JurorNotMatchedSigner,
    #[msg("Juror joining deadline, juror drawing deadline and dispute closure deadline not in a valid order with respect to current time.")]
    DisputeTimingsNotValid,
    #[msg("Join party not allowed outside of it's deadline.")]
    JoinPartyDeadlineViolated,
    #[msg("Can't approve a dispute when it's deadlines passed from current time.")]
    ApproveMissedDeadlineDispute,
    #[msg("Dispute has already been approved.")]
    DisputeAlreadyApproved,
    #[msg("Join juror not allowed outside of it's deadline.")]
    JoinJurorDeadlineViolated,
    #[msg("Draw juror not allowed outside of it's deadline.")]
    DrawJurorDeadlineViolated,
    #[msg("Voting not allowed outside of it's deadline.")]
    VoteDeadlineViolated,
    #[msg("Can't withdraw before drawing jurors.")]
    WithdrawBeforeJurorDrawProhibited,
    #[msg("Can't withdraw before closing dispute.")]
    WithdrawBeforeClosingDisputeProhibited,
    #[msg("Juror has already claimed the reward.")]
    JurorAlreadyClaimedReward,
    #[msg("Nothing to withdraw.")]
    NothingToWithdraw,
}
