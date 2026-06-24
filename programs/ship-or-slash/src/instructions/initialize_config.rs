use crate::state::ProtocolConfig;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + ProtocolConfig::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, ProtocolConfig>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeConfig>,
    treasury: Pubkey,
    sb_feed: Pubkey,
    slash_bps: u16,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.treasury = treasury;
    config.feed = sb_feed;
    config.slash_bps = slash_bps;
    config.bump = ctx.bumps.config;
    Ok(())
}
