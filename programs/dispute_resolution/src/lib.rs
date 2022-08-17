pub mod errors;
pub mod processor;
pub mod state;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, spl_token::instruction::AuthorityType, SetAuthority, Transfer};
use errors::CourtError;
use metaplex_token_metadata::state::Metadata;
use processor::*;

declare_id!("8rLpdGuKpqPRYF9odc1ken4AnxQfTF1tiXUTM2zJDXQ1");

#[program]
pub mod dispute_resolution {
    // TODO security: check for other instructions in this tx
    // TODO check mutation of disputes in ctx
    use super::*;
    // TODO initialize treasury account
    // TODO currently we don't support automation of transferring assets which there's claim
    pub fn initialize_settings(
        ctx: Context<InitializeSettings>,
        admin: Pubkey,
        juror_creator: Pubkey,
        raise_dispute_fee: u64,
    ) -> Result<()> {
        // Check if settings had been created
        if ctx.accounts.settings.master_admin != System::id() {
            // TODO check for default value of master admin
            return Err(CourtError::SettingsAlreadyCreated.into());
        }

        // Create treasury authority for court treasury token account
        let (treasury_authority, _treasury_authority_bump) = Pubkey::find_program_address(
            &[state::COURT_TREASURY_AUTHORITY_PDA_SEED],
            ctx.program_id,
        );
        token::set_authority(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                SetAuthority {
                    account_or_mint: ctx
                        .accounts
                        .court_treasury_token_account
                        .to_account_info()
                        .clone(),
                    current_authority: ctx.accounts.admin.to_account_info().clone(),
                },
            ),
            AuthorityType::AccountOwner,
            Some(treasury_authority),
        )?;

        // Set the settings
        ctx.accounts.settings.master_admin = ctx.accounts.admin.to_account_info().key();
        ctx.accounts.settings.admin = admin;
        ctx.accounts.settings.court_token = ctx.accounts.mint.to_account_info().key();
        ctx.accounts.settings.juror_creator = juror_creator;
        ctx.accounts.settings.court_treasury_token_account = ctx
            .accounts
            .court_treasury_token_account
            .to_account_info()
            .key();
        ctx.accounts.settings.raise_dispute_fee = raise_dispute_fee;

        Ok(())
    }

    pub fn set_settings(ctx: Context<SetSettings>, settings: crate::state::Settings) -> Result<()> {
        // Set the settings
        ctx.accounts.settings.master_admin = settings.master_admin;
        ctx.accounts.settings.admin = settings.admin;
        ctx.accounts.settings.juror_creator = settings.juror_creator;
        ctx.accounts.settings.raise_dispute_fee = settings.raise_dispute_fee;

        Ok(())
    }

    pub fn raise_dispute(
        ctx: Context<RaiseDispute>,
        dispute_data: crate::state::Dispute,
    ) -> Result<()> {
        // TODO do not take the Dispute object because it's heavy! we should use an alternative sol'n as we have many NULL values!
        if dispute_data.applicants[0].address != ctx.accounts.payer.to_account_info().key() {
            return Err(CourtError::PayerNotMatchFirstApplicantParty.into());
        }

        let clock: Clock = Clock::get().unwrap();
        if !(clock.unix_timestamp <= dispute_data.join_juror_deadline
            && dispute_data.join_juror_deadline <= dispute_data.draw_juror_deadline
            && dispute_data.draw_juror_deadline <= dispute_data.closure_deadline)
        {
            return Err(CourtError::DisputeTimingsNotValid.into());
        }

        let mut total_share = 0;
        for applicant in dispute_data.applicants {
            total_share += applicant.share;
            ctx.accounts.dispute.applicants.push(state::Party {
                address: applicant.address,
                joined: false,
                share: applicant.share,
                evidence_uri: applicant.evidence_uri,
                fingerprint: [0; 32],
            });
        }
        if total_share != 100 {
            return Err(CourtError::SharesExceeded.into());
        }
        ctx.accounts.dispute.applicants[0].joined = true;

        total_share = 0;
        for respondent in dispute_data.respondents {
            total_share += respondent.share;
            ctx.accounts.dispute.respondents.push(state::Party {
                address: respondent.address,
                joined: false,
                share: respondent.share,
                evidence_uri: respondent.evidence_uri,
                fingerprint: [0; 32],
            });
        }
        if total_share != 100 {
            return Err(CourtError::SharesExceeded.into());
        }

        ctx.accounts.dispute.init_time = clock.unix_timestamp;
        ctx.accounts.dispute.join_juror_deadline = dispute_data.join_juror_deadline;
        ctx.accounts.dispute.draw_juror_deadline = dispute_data.draw_juror_deadline;
        ctx.accounts.dispute.closure_deadline = dispute_data.closure_deadline;
        ctx.accounts.dispute.ready_jurors = 1;
        ctx.accounts.dispute.status = state::DisputeStatus::Initialized;

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.payer_token_account.to_account_info(),
                    to: ctx.accounts.court_treasury_token_account.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            ),
            ctx.accounts.settings.raise_dispute_fee,
        )?;
        Ok(())
    }

    pub fn join_party(ctx: Context<JoinParty>, evidence_uri: String) -> Result<()> {
        let clock: Clock = Clock::get().unwrap();
        // TODO add fingerprint of evidence
        if clock.unix_timestamp >= ctx.accounts.dispute.join_juror_deadline
            || ctx.accounts.dispute.status != state::DisputeStatus::Initialized
        {
            return Err(CourtError::JoinPartyDeadlineViolated.into());
        }

        let dispute: &mut Account<state::Dispute> = &mut ctx.accounts.dispute;
        for applicant in &mut dispute.applicants {
            // TODO do we need to check for percent of shares?
            if (*applicant).address == ctx.accounts.payer.to_account_info().key() {
                (*applicant).joined = true;
                (*applicant).evidence_uri = evidence_uri.clone();
                return Ok(());
            }
        }
        for respondent in &mut dispute.respondents {
            // TODO do we need to check for percent of shares?
            if (*respondent).address == ctx.accounts.payer.to_account_info().key() {
                (*respondent).joined = true;
                (*respondent).evidence_uri = evidence_uri.clone();
                return Ok(());
            }
        }

        return Err(CourtError::NoJoinAuthorize.into());
    }

    pub fn approve_dispute(
        ctx: Context<ApproveDispute>,
        dispute_value: u64,
        required_stake_amount: u64,
    ) -> Result<()> {
        let clock: Clock = Clock::get().unwrap();
        if clock.unix_timestamp >= ctx.accounts.dispute.join_juror_deadline {
            return Err(CourtError::ApproveMissedDeadlineDispute.into());
        }

        if ctx.accounts.dispute.status != state::DisputeStatus::Initialized {
            return Err(CourtError::DisputeAlreadyApproved.into());
        }

        ctx.accounts.dispute.status = state::DisputeStatus::Approved;
        ctx.accounts.dispute.dispute_value = dispute_value;
        ctx.accounts.dispute.required_stake_amount = required_stake_amount;
        Ok(())
    }

    pub fn join_juror(ctx: Context<JoinJuror>) -> Result<()> {
        // TODO duplicate juror
        // Check if NFT metadata is initialized (since we are using AccountInfo)
        if ctx.accounts.juror_nft_metadata_account.data_is_empty() {
            return Err(CourtError::MetadataNotInitialized.into());
        }
        // Check if NFT Master Edition is initialized (since we are using AccountInfo)
        if ctx.accounts.juror_nft_metadata_account.data_is_empty() {
            return Err(CourtError::MasterEditionNotInitialized.into());
        }

        let clock: Clock = Clock::get().unwrap();
        if clock.unix_timestamp >= ctx.accounts.dispute.join_juror_deadline
            || ctx.accounts.dispute.status != state::DisputeStatus::Approved
        {
            return Err(CourtError::JoinJurorDeadlineViolated.into());
        }

        // Check if juror NFT is created by MetaCourt authorized creator
        let metadata = &mut Metadata::from_account_info(&ctx.accounts.juror_nft_metadata_account)?;
        if metadata.data.creators.as_ref().unwrap()[0].address
            != ctx.accounts.settings.juror_creator
            || !(metadata.data.creators.as_ref().unwrap()[0].verified)
        {
            return Err(CourtError::NFTNotValid.into());
        }

        // Stake juror's COURT tokens into MetaCourt treasury
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.juror_token_account.to_account_info(),
                    to: ctx.accounts.court_treasury_token_account.to_account_info(),
                    authority: ctx.accounts.juror.to_account_info(),
                },
            ),
            ctx.accounts.dispute.required_stake_amount,
        )?;

        // Add juror to the list of ready jurors
        ctx.accounts.juror_reservation_entry.address = ctx.accounts.juror.to_account_info().key();
        ctx.accounts.dispute.ready_jurors += 1;

        Ok(())
    }

    pub fn draw_jurors(ctx: Context<DrawJurors>, jurors: Vec<crate::state::Juror>) -> Result<()> {
        // TODO determine number of jurors
        // TODO can we check if jurors have been declared as ready
        if jurors.len() == 7 {
            return Err(CourtError::JurorNumbersNotCorrect.into());
        }

        let clock: Clock = Clock::get().unwrap();
        if clock.unix_timestamp >= ctx.accounts.dispute.draw_juror_deadline
            || clock.unix_timestamp <= ctx.accounts.dispute.join_juror_deadline
            || ctx.accounts.dispute.status != state::DisputeStatus::Approved
        {
            return Err(CourtError::DrawJurorDeadlineViolated.into());
        }

        let dispute: &mut Account<state::Dispute> = &mut ctx.accounts.dispute;
        for i in 0..jurors.len() {
            dispute.jurors[i] = state::Juror {
                address: jurors[i].address,
                opinion: state::JurorOpinion::None,
                claimed_reward: false,
            };
        }
        // Change status to started so that remaining jurors can claim their stake
        dispute.status = state::DisputeStatus::Started;

        Ok(())
    }

    pub fn cast_vote(
        ctx: Context<CastVote>,
        _juror_id: u16,
        juror_opinion: crate::state::JurorOpinion,
    ) -> Result<()> {
        let clock: Clock = Clock::get().unwrap();
        if clock.unix_timestamp >= ctx.accounts.dispute.closure_deadline
            || ctx.accounts.dispute.status != state::DisputeStatus::Started
        {
            return Err(CourtError::VoteDeadlineViolated.into());
        }

        if ctx.accounts.juror_reservation_entry.address
            != ctx.accounts.payer.to_account_info().key()
        {
            return Err(CourtError::JurorNotMatchedSigner.into());
        }

        let dispute: &mut Account<state::Dispute> = &mut ctx.accounts.dispute;
        for juror in &mut dispute.jurors {
            if juror.address == ctx.accounts.payer.to_account_info().key() {
                juror.opinion = juror_opinion.clone();
                // We don't break here to support weighted votes!
            }
        }

        Ok(())
    }

    pub fn claim_stake(ctx: Context<ClaimStake>, _juror_id: u16) -> Result<()> {
        if ctx.accounts.juror_reservation_entry.address
            != ctx.accounts.juror.to_account_info().key()
        {
            return Err(CourtError::JurorNotMatchedSigner.into());
        }

        let clock: Clock = Clock::get().unwrap();
        let dispute_closure_deadline = ctx.accounts.dispute.closure_deadline.clone();
        let dispute: &mut Account<state::Dispute> = &mut ctx.accounts.dispute;
        let mut tokens_to_be_transferred: u64 = 0;

        if clock.unix_timestamp > dispute.draw_juror_deadline {
            if dispute.status == state::DisputeStatus::Started {
                // This juror might has or might not has been selected
                // Calculating number of votes for each party and if the juror was selected
                let mut applicant_vote_ctr = 0;
                let mut respondent_vote_ctr = 0;
                let mut abstention_vote_ctr = 0;
                let mut juror_selected = false;
                for juror in &mut dispute.jurors {
                    if juror.opinion == state::JurorOpinion::Applicant {
                        applicant_vote_ctr += 1;
                    } else if juror.opinion == state::JurorOpinion::Respondent {
                        respondent_vote_ctr += 1;
                    } else if juror.opinion == state::JurorOpinion::None {
                        abstention_vote_ctr += 1;
                    }
                    if juror.address == ctx.accounts.juror.to_account_info().key() {
                        // This juror has been selected
                        juror_selected = true;
                        if clock.unix_timestamp > dispute_closure_deadline {
                            // Dispute finished, so split the rewards
                            if juror.claimed_reward {
                                // Juror has already claimed the reward
                                return Err(CourtError::JurorAlreadyClaimedReward.into());
                            } else {
                                // TODO Calculate the reward
                                juror.claimed_reward = true;
                            }
                        } else {
                            // Dispute hasn't closed, can't withdraw
                            return Err(CourtError::WithdrawBeforeClosingDisputeProhibited.into());
                        }
                    }
                }

                if !juror_selected {
                    // This juror has not been selected
                    tokens_to_be_transferred = dispute.required_stake_amount;
                }
            } else {
                // Dispute did not start, it might be because of insufficient number of interested jurors
                tokens_to_be_transferred = dispute.required_stake_amount;
            }
        } else {
            return Err(CourtError::WithdrawBeforeJurorDrawProhibited.into());
        }

        if tokens_to_be_transferred == 0 {
            return Err(CourtError::NothingToWithdraw.into());
        }

        // Send tokens into Juror's COURT token account
        let (_treasury_authority, treasury_authority_bump) = Pubkey::find_program_address(
            &[state::COURT_TREASURY_AUTHORITY_PDA_SEED],
            ctx.program_id,
        );

        let treasury_authority_seeds = &[
            &state::COURT_TREASURY_AUTHORITY_PDA_SEED[..],
            &[treasury_authority_bump],
        ];

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.court_treasury_token_account.to_account_info(),
                    to: ctx.accounts.juror_token_account.to_account_info(),
                    authority: ctx.accounts.treasury_authority.to_account_info(),
                },
                &[&treasury_authority_seeds[..]],
            ),
            tokens_to_be_transferred,
        )?;

        Ok(())
    }
}
