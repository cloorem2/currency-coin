use {
    anchor_lang::prelude::*,
    anchor_spl::token,
};
use crate::create_mint_auth::MintAuth;

pub fn crank1(
    ctx: Context<Crank1>,
    mint_auth_bump: u8,
    _cc_mint_bump: u8,
    _ccb0_mint_bump: u8,
    _ccb1_mint_bump: u8,
    _ccs0_mint_bump: u8,
) -> Result<()> {
    assert_eq!(ctx.accounts.mint_authority.maturity_state, 1);
    ctx.accounts.mint_authority.cmod =
        (ctx.accounts.mint_authority.isum - 1.0) / 2.0;

    ctx.accounts.mint_authority.rmod = (ctx.accounts.cc_token.amount
        - ctx.accounts.ccb0_token.amount) as f64
        / ctx.accounts.ccb0_token.amount as f64
        - ctx.accounts.mint_authority.cmod;
    if ctx.accounts.mint_authority.rmod < 1.0 {
        ctx.accounts.mint_authority.rmod = 1.0;
    } else {
        ctx.accounts.mint_authority.imod /=
            ctx.accounts.mint_authority.rmod
            + ctx.accounts.mint_authority.cmod;
    }
    ctx.accounts.mint_authority.isum = 1.0;

    if ctx.accounts.ccs0_mint.supply == ctx.accounts.ccs0_token.amount {
        ctx.accounts.mint_authority.smod = 1.0;
    } else {
        ctx.accounts.mint_authority.smod *=
            ctx.accounts.ccs0_token.amount as f64
            / ctx.accounts.ccb0_token.amount as f64;
    }

    let f0 = (ctx.accounts.cc_token.amount
        - ctx.accounts.ccb0_token.amount) as f64;
        // * ctx.accounts.ccb0_token.amount as f64).sqrt();
    let mut x0 = f0.floor() as u64;
    if x0 < ctx.accounts.ccb0_token.amount {
        x0 = ctx.accounts.ccb0_token.amount;
    }
    let x1 = ctx.accounts.cc_token.amount
        - ctx.accounts.ccb0_token.amount - f0.ceil() as u64;
    if x1 > 0 {
        token::burn(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Burn {
                    mint: ctx.accounts.cc_mint.to_account_info(),
                    from: ctx.accounts.cc_token.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
                &[&[
                    b"mint_auth_",
                    &[mint_auth_bump],
                ]]
            ), x1,
        )?;
    }
    if x0 > ctx.accounts.ccb0_token.amount {
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.cc_mint.to_account_info(),
                    to: ctx.accounts.cc_token.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
                &[&[
                    b"mint_auth_",
                    &[mint_auth_bump],
                ]]
            ), x0 - ctx.accounts.ccb0_token.amount,
        )?;
    }
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.ccb1_mint.to_account_info(),
                to: ctx.accounts.ccb1_token.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[&[
                b"mint_auth_",
                &[mint_auth_bump],
            ]]
        ), x0,
    )?;
    token::burn(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                mint: ctx.accounts.ccb0_mint.to_account_info(),
                from: ctx.accounts.ccb0_token.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[&[
                b"mint_auth_",
                &[mint_auth_bump],
            ]]
        ), ctx.accounts.ccb0_token.amount,
    )?;

    // let x2 = (rmod * ctx.accounts.mint_authority.ccs_amount).floor();
    if x0 < ctx.accounts.ccs0_token.amount {
        let x2 = ctx.accounts.ccs0_token.amount - x0;
        token::burn(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Burn {
                    mint: ctx.accounts.ccs0_mint.to_account_info(),
                    from: ctx.accounts.ccs0_token.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
                &[&[
                    b"mint_auth_",
                    &[mint_auth_bump],
                ]]
            ), x2,
        )?;
    } else if x0 > ctx.accounts.ccs0_token.amount {
        let x2 = x0 - ctx.accounts.ccs0_token.amount;
        token::mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.ccs0_mint.to_account_info(),
                    to: ctx.accounts.ccs0_token.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
                &[&[
                    b"mint_auth_",
                    &[mint_auth_bump],
                ]]
            ), x2,
        )?;
    }

    ctx.accounts.mint_authority.maturity_state = 2;
    Ok(())
}

#[derive(Accounts)]
#[instruction(
    mint_auth_bump: u8,
    cc_mint_bump: u8,
    ccb0_mint_bump: u8,
    ccb1_mint_bump: u8,
    ccs0_mint_bump: u8,
)]
pub struct Crank1<'info> {
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
        seeds = [ b"ccs0_mint_" ],
        bump = ccs0_mint_bump
    )]
    pub ccs0_mint: Account<'info, token::Mint>,

    #[account(mut,
        associated_token::mint = cc_mint,
        associated_token::authority = mint_authority,
    )]
    pub cc_token: Box<Account<'info, token::TokenAccount>>,
    #[account(mut,
        associated_token::mint = ccb0_mint,
        associated_token::authority = mint_authority,
    )]
    pub ccb0_token: Box<Account<'info, token::TokenAccount>>,
    #[account(mut,
        associated_token::mint = ccb1_mint,
        associated_token::authority = mint_authority,
    )]
    pub ccb1_token: Box<Account<'info, token::TokenAccount>>,
    #[account(mut,
        associated_token::mint = ccs0_mint,
        associated_token::authority = mint_authority,
    )]
    pub ccs0_token: Box<Account<'info, token::TokenAccount>>,
    pub token_program: Program<'info, token::Token>,
}
