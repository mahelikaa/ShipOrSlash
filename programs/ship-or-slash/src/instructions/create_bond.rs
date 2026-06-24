use crate::state::Bond;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct CreateBond<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + Bond::INIT_SPACE,
        seeds = [b"bond",  owner.key().as_ref()],
        bump,
    )]
    pub bond: Account<'info, Bond>,

    /// CHECK: vault pda, holds lamports only
    #[account(
    init,
    payer = owner,
    space = 0,
    seeds = [b"vault", owner.key().as_ref()],
    bump,
)]
    pub vault: UncheckedAccount<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateBond>,
    github_username: String,
    target: u32,
    deadline: i64,
    lamports: u64,
) -> Result<()> {
    let bond = &mut ctx.accounts.bond;
    bond.owner = ctx.accounts.owner.key();
    bond.github_username = github_username;
    bond.target = target;
    bond.deadline = deadline;
    bond.lamports = lamports;
    bond.settled = false;
    bond.bump = ctx.bumps.bond;
    bond.vault_bump = ctx.bumps.vault;

    transfer(
        CpiContext::new(
            ctx.accounts.system_program.key(),
            Transfer {
                from: ctx.accounts.owner.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
            },
        ),
        lamports,
    )?;

    Ok(())
}
