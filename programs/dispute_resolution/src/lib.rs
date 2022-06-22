pub mod state;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey;
use anchor_spl::token::{ self, Mint, TokenAccount, Transfer };
use crate::state::{ Dispute, Party};

declare_id!("8rLpdGuKpqPRYF9odc1ken4AnxQfTF1tiXUTM2zJDXQ1");
const COURT_TOKEN: Pubkey = pubkey!("7ts4zibEhu1rL6CBXyGopZ7PFRLL8gPFKkdBQNAQCM6W");
const COURT_TREASURY_TOKEN_ACCOUNT: Pubkey = pubkey!("7ts4zibEhu1rL6CBXyGopZ7PFRLL8gPFKkdBQNAQCM6W");
const RAISE_DISPUTE_FEE: u64 = 1; // TODO fee to create dispute

#[program]
pub mod dispute_resolution {
    use super::*;

    pub fn raise_dispute(ctx: Context<RaiseDispute>, dispute_data: Dispute) -> Result<()> { // TODO do not take the Dispute object because it's heavy! we should use an alternative sol'n as we have many NULL values!
        if dispute_data.applicants[0].address != ctx.accounts.payer.to_account_info().key() {
            // Raise an error
        }
        
        let mut total_share = 0;
        for applicant in dispute_data.applicants { // TODO remove first one and use the payer instead
            total_share += applicant.share;
            ctx.accounts.dispute.applicants.push(
                Party {
                    address: applicant.address,
                    joined: false,
                    share: applicant.share,
                    uri: String::from(""),
                    fingerprint: [0; 32]
                }
            );
        }
        if total_share != 100 {
            // Raise an error
        }
        ctx.accounts.dispute.applicants[0].joined = true;

        total_share = 0;
        for respondent in dispute_data.respondents {
            total_share += respondent.share;
            ctx.accounts.dispute.respondents.push(
                Party {
                    address: respondent.address,
                    joined: false,
                    share: respondent.share,
                    uri: String::from(""),
                    fingerprint: [0; 32]
                }
            );
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
                    authority: ctx.accounts.payer.to_account_info()
                }
            ), 
            RAISE_DISPUTE_FEE
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(dispute_data: Dispute)]
pub struct RaiseDispute<'info> {
    #[account(
        init, 
        payer = payer, 
        space = state::DISPUTE_SIZE + ((dispute_data.applicants.len() + dispute_data.respondents.len()) * state::PARTY_SIZE))]
    pub dispute: Account<'info, state::Dispute>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut, 
        constraint = payer_token_account.amount >= RAISE_DISPUTE_FEE,
        constraint = payer_token_account.mint == mint.to_account_info().key(),
        constraint = payer_token_account.owner == payer.to_account_info().key()
    )]
    pub payer_token_account: Account<'info, TokenAccount>,
    #[account(address = COURT_TREASURY_TOKEN_ACCOUNT)]
    pub court_treasury_token_account: Account<'info, TokenAccount>,
    #[account(address = COURT_TOKEN)]
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, anchor_spl::token::Token>,
}
