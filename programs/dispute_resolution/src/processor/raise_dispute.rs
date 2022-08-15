use crate::state;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

#[derive(Accounts)]
#[instruction(dispute_data: state::Dispute)]
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
        constraint = payer_token_account.amount >= state::RAISE_DISPUTE_FEE,
        constraint = payer_token_account.mint == mint.to_account_info().key(),
        constraint = payer_token_account.owner == payer.to_account_info().key()
    )]
    pub payer_token_account: Account<'info, TokenAccount>,
    #[account(address = state::COURT_TREASURY_TOKEN_ACCOUNT)]
    pub court_treasury_token_account: Account<'info, TokenAccount>,
    #[account(address = state::COURT_TOKEN)]
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, anchor_spl::token::Token>,
}
