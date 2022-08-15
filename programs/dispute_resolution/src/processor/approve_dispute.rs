use crate::state;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ApproveDispute<'info> {
    #[account(mut)]
    pub dispute: Account<'info, state::Dispute>,
    #[account(mut, address=state::ADMIN)]
    pub payer: Signer<'info>,
}
