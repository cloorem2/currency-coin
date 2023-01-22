use {
    anchor_lang::prelude::*,
    anchor_spl::token,
};
use crate::create_mint_auth::MintAuth;

pub fn redeem_bonds1(
    ctx: Context<RedeemBonds1>,
    mint_auth_bump: u8,
    _cc_mint_bump: u8,
    _ccb0_mint_bump: u8,
    _ccb1_mint_bump: u8,
) -> Result<()> {
    assert_eq!(ctx.accounts.mint_authority.maturity_state, 0);
    let x0 = (ctx.accounts.owner_ccb1_token.amount as f64
        * ctx.accounts.mint_authority.rmod).floor() as u64;
    let x1 = (ctx.accounts.owner_ccb1_token.amount as f64
        * ctx.accounts.mint_authority.cmod).floor() as u64;
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.ccb0_mint.to_account_info(),
                to: ctx.accounts.owner_ccb0_token.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[&[
                b"mint_auth_",
                &[mint_auth_bump],
            ]]
        ), x0,
    )?;
    if x1 > 0 {
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.cc_mint.to_account_info(),
                    to: ctx.accounts.owner_cc_token.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
                &[&[
                    b"mint_auth_",
                    &[mint_auth_bump],
                ]]
            ), x1,
        )?;
    }
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                mint: ctx.accounts.ccb1_mint.to_account_info(),
                from: ctx.accounts.owner_ccb1_token.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ), ctx.accounts.owner_ccb1_token.amount,
    )?;
    Ok(())
}


#[derive(Accounts)]
#[instruction(
    mint_auth_bump: u8,
    cc_mint_bump: u8,
    ccb0_mint_bump: u8,
    ccb1_mint_bump: u8,
)]
pub struct RedeemBonds1<'info> {
    #[account(mut,
        seeds = [ b"mint_auth_" ],
        bump = mint_auth_bump
    )]
    pub mint_authority: Account<'info, MintAuth>,

    #[account(mut,
        seeds = [ b"cc_mint_" ],
        bump = cc_mint_bump
    )]
    pub cc_mint: Account<'info, token::Mint>,
    #[account(mut,
        seeds = [ b"ccb0_mint_" ],
        bump = ccb0_mint_bump
    )]
    pub ccb0_mint: Account<'info, token::Mint>,
    #[account(mut,
        seeds = [ b"ccb1_mint_" ],
        bump = ccb1_mint_bump
    )]
    pub ccb1_mint: Account<'info, token::Mint>,

    #[account(mut,
        associated_token::mint = cc_mint,
        associated_token::authority = owner,
    )]
    pub owner_cc_token: Box<Account<'info, token::TokenAccount>>,
    #[account(mut,
        associated_token::mint = ccb0_mint,
        associated_token::authority = owner,
    )]
    pub owner_ccb0_token: Box<Account<'info, token::TokenAccount>>,
    #[account(mut,
        associated_token::mint = ccb1_mint,
        associated_token::authority = owner,
    )]
    pub owner_ccb1_token: Box<Account<'info, token::TokenAccount>>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub token_program: Program<'info, token::Token>,
}
