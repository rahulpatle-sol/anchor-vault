use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};

declare_id!("A2NKgCASKEFVLMLZ2rjghx65R9mkMQHom7KEo8PzPihn");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.vault_state;
        state.vault_bump = ctx.bumps.vault;
        state.state_bump = ctx.bumps.vault_state;
        msg!(
            "Vault initialized with bumps: vault={}, state={}",
            state.vault_bump,
            state.state_bump
        );
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.from.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        token::transfer(cpi_ctx, amount).map_err(Into::into)?;
        msg!("Deposited {} tokens to vault", amount);
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let seeds = &[
            b"vault",
            ctx.accounts.user.key.as_ref(),
            &[ctx.accounts.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                authority: ctx.accounts.vault.to_account_info(),
            },
            signer_seeds,
        );

        token::transfer(cpi_ctx, amount).map_err(Into::into)?;
        msg!("Withdrawn {} tokens from vault", amount);
        Ok(())
    }
}
