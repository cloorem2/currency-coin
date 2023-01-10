use {
    anchor_lang::prelude::*,
    anchor_spl::{
        token,
        associated_token,
    },
};
use crate::create_mint_auth::MintAuth;


pub fn mint_to_your_wallet(
    ctx: Context<MintToYourWallet>,
    cc_mint_bump: u8,
    ccb0_mint_bump: u8,
    mint_auth_bump: u8,
) -> Result<()> {
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.cc_mint_account.to_account_info(),
                to: ctx.accounts.my_cc_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[&[
                b"mint_auth_",
                &[mint_auth_bump],
            ]]
        ), 2,
    )?;
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.ccb0_mint_account.to_account_info(),
                to: ctx.accounts.my_ccb0_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[&[
                b"mint_auth_",
                &[mint_auth_bump],
            ]]
        ), 2,
    )?;
    Ok(())
}


#[derive(Accounts)]
#[instruction(cc_mint_bump: u8, ccb0_mint_bump: u8, mint_auth_bump: u8)]
pub struct MintToYourWallet<'info> {
    #[account( mut,
        mint::decimals = 0,
        mint::authority = mint_authority.key(),
        seeds = [ b"cc_mint_" ],
        bump = cc_mint_bump
    )]
    pub cc_mint_account: Account<'info, token::Mint>,
    #[account( mut,
        mint::decimals = 0,
        mint::authority = mint_authority.key(),
        seeds = [ b"ccb0_mint_" ],
        bump = ccb0_mint_bump
    )]
    pub ccb0_mint_account: Account<'info, token::Mint>,
    #[account( mut,
        seeds = [ b"mint_auth_" ],
        bump = mint_auth_bump
    )]
    pub mint_authority: Account<'info, MintAuth>,
    #[account( init,
        payer = payer,
        associated_token::mint = cc_mint_account,
        associated_token::authority = payer,
    )]
    pub my_cc_account: Account<'info, token::TokenAccount>,
    #[account( init,
        payer = payer,
        associated_token::mint = ccb0_mint_account,
        associated_token::authority = payer,
    )]
    pub my_ccb0_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}
