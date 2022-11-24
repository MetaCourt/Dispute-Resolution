use crate::state;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RemoveJurorEntry<'info> {
    #[account(mut, close = payer)]
    pub juror_entry: Account<'info, state::JurorReservationEntry>,
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
