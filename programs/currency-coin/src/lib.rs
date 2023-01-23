use anchor_lang::prelude::*;
pub mod instructions;
use instructions::*;

declare_id!("A3mhZsPAjroa8oQokfjGKNNNrKwbpz5ZwszHduYs6hz6");

#[program]
pub mod currency_coin {
    use super::*;

    pub fn create_mint_auth(
        ctx: Context<CreateMintAuth>,
    ) -> Result<()> {
        create_mint_auth::create_mint_auth(ctx)
    }

    pub fn crank0(
        ctx: Context<Crank0>,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb0_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        crank0::crank0(
            ctx, mint_auth_bump,
            cc_mint_bump,
            ccb0_mint_bump,
            ccs0_mint_bump
        )
    }

    pub fn crank2(
        ctx: Context<Crank2>,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb1_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        crank2::crank2(
            ctx, mint_auth_bump,
            cc_mint_bump,
            ccb1_mint_bump,
            ccs0_mint_bump
        )
    }

    pub fn crank1(
        ctx: Context<Crank1>,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb0_mint_bump: u8,
        ccb1_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        crank1::crank1(
            ctx,
            mint_auth_bump,
            cc_mint_bump,
            ccb0_mint_bump,
            ccb1_mint_bump,
            ccs0_mint_bump,
        )
    }

    pub fn crank3(
        ctx: Context<Crank3>,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb0_mint_bump: u8,
        ccb1_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        crank3::crank3(
            ctx,
            mint_auth_bump,
            cc_mint_bump,
            ccb0_mint_bump,
            ccb1_mint_bump,
            ccs0_mint_bump,
        )
    }

    pub fn buy_bonds0(
        ctx: Context<BuyBonds0>,
        amount: u64,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb0_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        buy_bonds0::buy_bonds0(
            ctx, amount, mint_auth_bump,
            cc_mint_bump,
            ccb0_mint_bump,
            ccs0_mint_bump,
        )
    }

    pub fn buy_bonds1(
        ctx: Context<BuyBonds1>,
        amount: u64,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb1_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        buy_bonds1::buy_bonds1(
            ctx, amount, mint_auth_bump,
            cc_mint_bump,
            ccb1_mint_bump,
            ccs0_mint_bump,
        )
    }

    pub fn buy_shorts0(
        ctx: Context<BuyShorts0>,
        amount: u64,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb0_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        buy_shorts0::buy_shorts0(
            ctx, amount, mint_auth_bump,
            cc_mint_bump,
            ccb0_mint_bump,
            ccs0_mint_bump,
        )
    }
    pub fn buy_shorts1(
        ctx: Context<BuyShorts1>,
        amount: u64,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb1_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        buy_shorts1::buy_shorts1(
            ctx, amount, mint_auth_bump,
            cc_mint_bump,
            ccb1_mint_bump,
            ccs0_mint_bump,
        )
    }

    pub fn sell_bonds0(
        ctx: Context<SellBonds0>,
        amount: u64,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb0_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        sell_bonds0::sell_bonds0(
            ctx, amount, mint_auth_bump,
            cc_mint_bump,
            ccb0_mint_bump,
            ccs0_mint_bump,
        )
    }

    pub fn sell_bonds1(
        ctx: Context<SellBonds1>,
        amount: u64,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb1_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        sell_bonds1::sell_bonds1(
            ctx, amount, mint_auth_bump,
            cc_mint_bump,
            ccb1_mint_bump,
            ccs0_mint_bump,
        )
    }

    pub fn sell_shorts0(
        ctx: Context<SellShorts0>,
        amount: u64,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb0_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        sell_shorts0::sell_shorts0(
            ctx, amount, mint_auth_bump,
            cc_mint_bump,
            ccb0_mint_bump,
            ccs0_mint_bump,
        )
    }

    pub fn sell_shorts1(
        ctx: Context<SellShorts1>,
        amount: u64,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb1_mint_bump: u8,
        ccs0_mint_bump: u8,
    ) -> Result<()> {
        sell_shorts1::sell_shorts1(
            ctx, amount, mint_auth_bump,
            cc_mint_bump,
            ccb1_mint_bump,
            ccs0_mint_bump,
        )
    }

    pub fn redeem_bonds0(
        ctx: Context<RedeemBonds0>,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb0_mint_bump: u8,
        ccb1_mint_bump: u8,
    ) -> Result<()> {
        redeem_bonds0::redeem_bonds0(
            ctx,
            mint_auth_bump,
            cc_mint_bump,
            ccb0_mint_bump,
            ccb1_mint_bump,
        )
    }

    pub fn redeem_bonds1(
        ctx: Context<RedeemBonds1>,
        mint_auth_bump: u8,
        cc_mint_bump: u8,
        ccb0_mint_bump: u8,
        ccb1_mint_bump: u8,
    ) -> Result<()> {
        redeem_bonds1::redeem_bonds1(
            ctx,
            mint_auth_bump,
            cc_mint_bump,
            ccb0_mint_bump,
            ccb1_mint_bump,
        )
    }
}
