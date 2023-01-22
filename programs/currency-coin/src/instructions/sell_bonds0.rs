use {
    anchor_lang::prelude::*,
    anchor_spl::token,
};
use crate::create_mint_auth::MintAuth;

pub fn sell_bonds0(
    ctx: Context<SellBonds0>,
    // the most cc buyer is sending, we leave the dust behind
    amount: u64,
    mint_auth_bump: u8,
    _cc_mint_bump: u8,
    _ccb0_mint_bump: u8,
    _ccs0_mint_bump: u8,
) -> Result<()> {
    assert_eq!(ctx.accounts.mint_authority.maturity_state, 0);

    let r0 = ctx.accounts.ccb0_token.amount as f64;
    let r1 = (ctx.accounts.cc_token.amount
        - ctx.accounts.ccb0_token.amount) as f64;
    let r2 = r0 + amount as f64;
    let cc_to_owner = ((r2 * r1 - r0 * r1) / r2).floor() as u64;
    assert_eq!(cc_to_owner > 0, true);

    let r3 = r1 - cc_to_owner as f64;
    let b0_from_owner = ((r1 * r0 - r3 * r0) / r3).ceil() as u64;
    // this should be unnecessary but it's nice anyway
    assert_eq!(b0_from_owner <= amount, true);

    let sr2 = r0 + b0_from_owner as f64;
    let s0_to_burn = (ctx.accounts.ccs0_token.amount as f64 *
        (1.0 - (r0 / sr2).powf(ctx.accounts.mint_authority.smod))
    ).floor() as u64;

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.cc_token.to_account_info(),
                to: ctx.accounts.owner_cc_token.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            &[&[
                b"mint_auth_",
                &[mint_auth_bump],
            ]]
        ), cc_to_owner,
    )?;
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.owner_ccb0_token.to_account_info(),
                to: ctx.accounts.ccb0_token.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ), b0_from_owner,
    )?;
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
        ), b0_from_owner,
    )?;
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
        ), s0_to_burn,
    )?;

    Ok(())
}


#[derive(Accounts)]
#[instruction(
    amount: u64,
    mint_auth_bump: u8,
    cc_mint_bump: u8,
    ccb0_mint_bump: u8,
    ccs0_mint_bump: u8,
    // cc_mint: Pubkey,
    // ccb0_mint: Pubkey,
)]
pub struct SellBonds0<'info> {
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
        seeds = [ b"ccs0_mint_" ],
        bump = ccs0_mint_bump
    )]
    pub ccs0_mint: Account<'info, token::Mint>,

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
        associated_token::mint = ccs0_mint,
        associated_token::authority = mint_authority,
    )]
    pub ccs0_token: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub owner: Signer<'info>,
    // pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    // pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}
