use crate::state;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct JoinParty<'info> {
    #[account(mut)]
    pub dispute: Account<'info, state::Dispute>,
    #[account(mut)]
    pub payer: Signer<'info>,
}
