use crate::state;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

#[derive(Accounts)]
#[instruction(juror_id: u16)]
pub struct ClaimStake<'info> {
    #[account(
        seeds = [
            b"juror",
            dispute.to_account_info().key().as_ref(),
            juror_id.to_string().as_ref()
        ],
        bump,
    )]
    pub juror_reservation_entry: Account<'info, state::JurorReservationEntry>,
    #[account(mut)]
    pub dispute: Box<Account<'info, state::Dispute>>,
    // The owner of juror NFT
    #[account(mut)]
    pub juror: Signer<'info>,

    #[account(
        mut,
        constraint = juror_token_account.mint == mint.to_account_info().key(),
        constraint = juror_token_account.owner == juror.to_account_info().key()
    )]
    pub juror_token_account: Account<'info, TokenAccount>,
    #[account(address = state::COURT_TREASURY_TOKEN_ACCOUNT)]
    pub court_treasury_token_account: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because checks will be performed in token program when transferring tokens
    pub treasury_authority: AccountInfo<'info>,
    #[account(address = state::COURT_TOKEN)]
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, anchor_spl::token::Token>,
}
