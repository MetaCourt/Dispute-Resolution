pub mod errors;
pub mod processor;
pub mod state;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use errors::CourtError;
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
            // First one should be the payer
            // Raise an error
        }

        let clock: Clock = Clock::get().unwrap();
        if clock.unix_timestamp >= dispute_data.dispute_closure_timestamp {
            // Raise an error
            // TODO how should timings evaluated?
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
            // Raise an error
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
            // Raise an error
        }

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

        return Err(error!(CourtError::NoJoinAuthorize));
    }

    pub fn approve_dispute(ctx: Context<ApproveDispute>, dispute_value: u64) -> Result<()> {
        ctx.accounts.dispute.status = state::DisputeStatus::Approved;
        ctx.accounts.dispute.dispute_value = dispute_value;
        Ok(())
    }
}
