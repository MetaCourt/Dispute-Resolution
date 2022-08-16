use crate::state;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(juror_id: u16)]
pub struct CastVote<'info> {
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
    pub dispute: Account<'info, state::Dispute>,
    #[account(mut)]
    pub payer: Signer<'info>,
}
