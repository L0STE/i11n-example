use anchor_lang::prelude::*;

pub mod contexts;
use contexts::*;

pub mod i11n;
use i11n::*;

pub mod state;

declare_id!("2izLgwrneSriptaHAbTDLrYhum9SuyDb2t9zvM4nGo8m");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.deposit(deposit)?;
        ctx.accounts.save_escrow(seed, receive, &ctx.bumps)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close_vault()
    }

    // i11n Examples
    pub fn introspection_check(ctx: Context<IntrospectionCheck>) -> Result<()> {
        ctx.accounts.maker_check()
    }

    pub fn make_cpi(ctx: Context<MakeCpi>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.make_cpi(seed, deposit, receive)
    }
}