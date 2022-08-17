use crate::state;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

#[derive(Accounts)]
pub struct SetSettings<'info> {
    #[account(
        seeds = [
            state::SETTINGS_PDA
        ],
        bump,
    )]
    pub settings: Account<'info, state::Settings>,
    #[account(mut, address = settings.master_admin)]
    pub master_admin: Signer<'info>,
}
