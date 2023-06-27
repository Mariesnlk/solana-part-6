use crate::state::programConfig;
use crate::ADMIN;
use crate::SEED_PROGRAM_CONFIG;
use crate::USDC_MINT_PUBKEY;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct InitializeProgramConfig<'info> {
    #[account(init, seeds = [SEED_PROGRAM_CONFIG], bump, payer = authority, space = ProgramConfig::LEN)]
    pub program_config: Account<'info, programConfig::ProgramConfig>,
    #[account(token::mint = USDC_MINT_PUBKEY)]
    pub fee_destination: AccountInfo<'info, TokenAccount>,
    #[account(mut, address = ADMIN)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_program_config_handler(
    ctx: Context<InitializeProgramConfig>,
) -> Result<()> {
    let program_config = &mut ctx.accounts.program_config;
    program_config.admin = ctx.accounts.authority.key();
    program_config.fee_destination = ctx.accounts.fee_destination.key();
    program_config.fee_basis_points = 100;
    Ok(())
}

// ctx.accounts.program_config.admin = ctx.accounts.authority.key();
// ctx.accounts.program_config.fee_destination = ctx.accounts.fee_destination.key();
// ctx.accounts.program_config.fee_basis_points = 100;