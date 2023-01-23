use {
    anchor_lang::prelude::*,
    anchor_spl::token,
};
use crate::create_mint_auth::MintAuth;

pub fn crank0(
    ctx: Context<Crank0>,
    mint_auth_bump: u8,
    _cc_mint_bump: u8,
    _ccb0_mint_bump: u8,
    _ccs0_mint_bump: u8,
) -> Result<()> {
    assert_eq!(ctx.accounts.mint_authority.maturity_state,0);
    let pir = ctx.accounts.ccb0_token.amount as f64
        * ctx.accounts.mint_authority.imod;
    // let ir = ctx.accounts.mint_authority.ccb_amount
        // / ctx.accounts.mint_authority.cc0_amount
        // * ctx.accounts.mint_authority.imod;
    let ir = pir / (ctx.accounts.cc_token.amount
        - ctx.accounts.ccb0_token.amount) as f64;
    // let cc_to_mint = (ir * ctx.accounts.mint_authority.cc0_amount).floor();
    let mut cc_to_mint = pir.floor() as u64;
    if cc_to_mint < 1 { cc_to_mint = 1; }
    // ctx.accounts.mint_authority.imod *= ir + 1.0;
    ctx.accounts.mint_authority.imod *= 1.0 + cc_to_mint as f64
        / (ctx.accounts.cc_token.amount
            - ctx.accounts.ccb0_token.amount) as f64;
    ctx.accounts.mint_authority.isum *= 1.0 + cc_to_mint as f64
        / (ctx.accounts.cc_token.amount
            - ctx.accounts.ccb0_token.amount) as f64;
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
        ), cc_to_mint,
    )?;

    let s0_to_mint = (ir * ctx.accounts.ccs0_token.amount as f64).floor() as u64;
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
        ), s0_to_mint,
    )?;

    let pool0 = &mut ctx.accounts.mint_authority;
    // pool0.imod *= (pool0.cc0_amount + cc_to_mint) / pool0.cc0_amount;
    // ir is big by the rounding error, but i bet it doesn't matter
    // pool0.imod *= ir + 1.0;

    let clock: Clock = Clock::get().unwrap();
    // if ctx.accounts.cc_token.amount >= 3 * ctx.accounts.ccb0_token.amount {
    let spw = 60*60*24*7;
    if clock.unix_timestamp % spw < pool0.timestamp % spw {
        pool0.maturity_state = 1;
    }
    let dt = clock.unix_timestamp - pool0.timestamp;
    assert_eq!(dt >= 5, true);
    pool0.timestamp = clock.unix_timestamp;
    let ips = (ir / dt as f64) as f32;
    pool0.ima0 *= 100.0;
    pool0.ima0 += ips;
    pool0.ima0 /= 101.0;

    pool0.ima1 *= 10000.0;
    pool0.ima1 += ips;
    pool0.ima1 /= 10001.0;

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    mint_auth_bump: u8,
    cc_mint_bump: u8,
    ccb0_mint_bump: u8,
    ccs0_mint_bump: u8,
)]
pub struct Crank0<'info> {
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
    #[account(
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
        associated_token::authority = mint_authority,
    )]
    pub cc_token: Account<'info, token::TokenAccount>,
    #[account(
        associated_token::mint = ccb0_mint,
        associated_token::authority = mint_authority,
    )]
    pub ccb0_token: Account<'info, token::TokenAccount>,
    #[account(mut,
        associated_token::mint = ccs0_mint,
        associated_token::authority = mint_authority,
    )]
    pub ccs0_token: Account<'info, token::TokenAccount>,
    pub token_program: Program<'info, token::Token>,
}
