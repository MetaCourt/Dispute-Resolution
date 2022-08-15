pub mod processor;
pub mod state;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer};
use processor::*;

declare_id!("8rLpdGuKpqPRYF9odc1ken4AnxQfTF1tiXUTM2zJDXQ1");

#[program]
pub mod dispute_resolution {
    use super::*;

    pub fn raise_dispute(
        ctx: Context<RaiseDispute>,
        dispute_data: crate::state::Dispute,
    ) -> Result<()> {
        // TODO do not take the Dispute object because it's heavy! we should use an alternative sol'n as we have many NULL values!
        if dispute_data.applicants[0].address != ctx.accounts.payer.to_account_info().key() {
            // Raise an error
        }

        let mut total_share = 0;
        for applicant in dispute_data.applicants {
            // TODO remove first one and use the payer instead
            total_share += applicant.share;
            ctx.accounts.dispute.applicants.push(state::Party {
                address: applicant.address,
                joined: false,
                share: applicant.share,
                uri: String::from(""),
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
                uri: String::from(""),
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
}
