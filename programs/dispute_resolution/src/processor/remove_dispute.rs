use crate::state;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RemoveDispute<'info> {
    #[account(mut, close = payer)]
    pub dispute: Account<'info, state::Dispute>,
    #[account(mut, address = settings.admin)]
    pub payer: Signer<'info>,
    #[account(
        seeds = [
            state::SETTINGS_PDA
        ],
        bump,
    )]
    pub settings: Account<'info, state::Settings>,
}
