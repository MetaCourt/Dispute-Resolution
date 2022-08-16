pub mod errors;
pub mod processor;
pub mod state;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use errors::CourtError;
use metaplex_token_metadata::state::Metadata;
use processor::*;

declare_id!("8rLpdGuKpqPRYF9odc1ken4AnxQfTF1tiXUTM2zJDXQ1");

#[program]
pub mod dispute_resolution {
    use super::*;
    // TODO currently we don't support automation of transferring assets which there's claim
    pub fn raise_dispute(
        ctx: Context<RaiseDispute>,
        dispute_data: crate::state::Dispute,
    ) -> Result<()> {
        // TODO do not take the Dispute object because it's heavy! we should use an alternative sol'n as we have many NULL values!
        if dispute_data.applicants[0].address != ctx.accounts.payer.to_account_info().key() {
            return Err(CourtError::PayerNotMatchFirstApplicantParty.into());
        }

        let clock: Clock = Clock::get().unwrap();
        if clock.unix_timestamp >= dispute_data.dispute_closure_timestamp {
            // Raise an error
            // TODO how should timings evaluated?
            // TODO how should timings be? store starting time?
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
        ctx.accounts.dispute.dispute_closure_timestamp = dispute_data.dispute_closure_timestamp;
        ctx.accounts.dispute.ready_jurors = 1;

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.payer_token_account.to_account_info(),
                    to: ctx.accounts.court_treasury_token_account.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            ),
            state::RAISE_DISPUTE_FEE,
        )?;
        Ok(())
    }

    pub fn join_party(ctx: Context<JoinParty>, evidence_uri: String) -> Result<()> {
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
        ctx.accounts.dispute.status = state::DisputeStatus::Approved;
        ctx.accounts.dispute.dispute_value = dispute_value;
        ctx.accounts.dispute.required_stake_amount = required_stake_amount;
        Ok(())
    }

    pub fn join_juror(ctx: Context<JoinJuror>) -> Result<()> {
        // Check if NFT metadata is initialized (since we are using AccountInfo)
        if ctx.accounts.juror_nft_metadata_account.data_is_empty() {
            return Err(CourtError::MetadataNotInitialized.into());
        }
        // Check if NFT Master Edition is initialized (since we are using AccountInfo)
        if ctx.accounts.juror_nft_metadata_account.data_is_empty() {
            return Err(CourtError::MasterEditionNotInitialized.into());
        }
        // TODO time must be valid

        // Check if juror NFT is created by MetaCourt authorized creator
        let metadata = &mut Metadata::from_account_info(&ctx.accounts.juror_nft_metadata_account)?;
        if metadata.data.creators.as_ref().unwrap()[0].address != state::JUROR_CREATOR
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
}
