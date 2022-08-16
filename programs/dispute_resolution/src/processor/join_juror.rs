use crate::state;
use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, TokenAccount };

#[derive(Accounts)]
#[instruction(dispute_data: state::Dispute)]
pub struct JoinJuror<'info> {
    #[account(
        init,
        payer = juror,
        seeds = [
            b"juror", 
            dispute.to_account_info().key().as_ref(), 
            dispute.ready_jurors.to_string().as_ref()
        ],
        bump,
        space = state::JUROR_RESERVATION_ENTRY_SIZE
    )]
    pub juror_reservation_entry: Box<Account<'info, state::JurorReservationEntry>>,
    #[account(mut)]
    pub dispute: Box<Account<'info, state::Dispute>>,
    // The owner of juror NFT
    #[account(mut)]
    pub juror: Signer<'info>,
    // NFT account (mint of the NFT)
    pub juror_nft_mint: Account<'info, Mint>,
    // token account containing the NFT
    #[account(
        constraint = juror_nft_token_account.amount == 1,
        constraint = juror_nft_token_account.mint == juror_nft_mint.to_account_info().key(),
        constraint = juror_nft_token_account.owner == juror.to_account_info().key()
    )]
    pub juror_nft_token_account: Account<'info, TokenAccount>,
    /// CHECK: Metadata account of NFT, address checked, initialization checked in function
    #[account(
        seeds = [
            b"metadata", 
            token_metadata_program.key().as_ref(),
            juror_nft_mint.key().as_ref(),            
        ],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    pub juror_nft_metadata_account: AccountInfo<'info>,
    /// CHECK: Master Edition account of NFT, address checked, initialization checked in function
    #[account(
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            juror_nft_mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    pub juror_nft_master_edition_account: AccountInfo<'info>,

    #[account(
        mut, 
        constraint = juror_token_account.amount >= dispute.required_stake_amount,
        constraint = juror_token_account.mint == mint.to_account_info().key(),
        constraint = juror_token_account.owner == juror.to_account_info().key()
    )]
    pub juror_token_account: Account<'info, TokenAccount>,
    #[account(address = state::COURT_TREASURY_TOKEN_ACCOUNT)]
    pub court_treasury_token_account: Account<'info, TokenAccount>,
    #[account(address = state::COURT_TOKEN)]
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, anchor_spl::token::Token>,
    /// CHECK: Program account of metadata, ownership checked
    #[account(address = state::METADATA_PROGRAM_ID)]
    pub token_metadata_program: AccountInfo<'info>,
}
