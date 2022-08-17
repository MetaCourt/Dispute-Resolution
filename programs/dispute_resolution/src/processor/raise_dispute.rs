use crate::state;
use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, TokenAccount };

#[derive(Accounts)]
#[instruction(dispute_data: state::Dispute)]
pub struct RaiseDispute<'info> {
    #[account(
        init,
        payer = payer, 
        space = state::DISPUTE_SIZE + ((dispute_data.applicants.len() + dispute_data.respondents.len()) * state::PARTY_SIZE))]
    pub dispute: Box<Account<'info, state::Dispute>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut, 
        constraint = payer_token_account.amount >= settings.raise_dispute_fee,
        constraint = payer_token_account.mint == mint.to_account_info().key(),
        constraint = payer_token_account.owner == payer.to_account_info().key()
    )]
    pub payer_token_account: Account<'info, TokenAccount>,
    #[account(
        seeds = [
            state::SETTINGS_PDA
        ],
        bump,
    )]
    pub settings: Account<'info, state::Settings>,
    #[account(mut, address = settings.court_treasury_token_account)]
    pub court_treasury_token_account: Account<'info, TokenAccount>,
    #[account(address = settings.court_token)]
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, anchor_spl::token::Token>,
}
