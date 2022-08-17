use crate::state;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

#[derive(Accounts)]
pub struct InitializeSettings<'info> {
    #[account(
        init,
        payer = admin,
        seeds = [
            state::SETTINGS_PDA
        ],
        bump,
        space = state::SETTINGS_SIZE
    )]
    pub settings: Account<'info, state::Settings>,
    #[account(
        init,
        payer = admin,
        seeds = [
            state::SETTINGS_PDA
        ],
        bump,
        token::mint = mint,
        token::authority = admin,
    )]
    pub court_treasury_token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
