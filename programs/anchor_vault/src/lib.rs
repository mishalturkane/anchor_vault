#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::{prelude::*,system_program::{Transfer, transfer}};

declare_id!("DosyEdvk9k9MEMk1rxSScsgirh6TXn4hzH9aJrikT5im");
// Program ID: This declares the program's unique identifier, a crucial address on the Solana blockchain.

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
        
    }
    pub fn deposit(ctx: Context<Payment>, amount: u64)-> Result<()>{
        ctx.accounts.deposit(amount)
    }
    pub fn withdraw(ctx: Context<Payment>, amount: u64)-> Result<()>{
        ctx.accounts.withdraw(amount)
    }
    pub fn close(ctx: Context<CloseAccount>)-> Result<()>{
        ctx.accounts.close()
    }

}

#[derive(Accounts)]
pub struct Initialize <'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"state", user.key().as_ref()],
        bump,
        space = VaultState::INIT_SPACE
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        seeds = [b"vault", user.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
impl<'info> Initialize<'info>{
    pub fn initialize(&mut self, bump: &InitializeBumps)-> Result<()>{
        self.vault_state.vault_bump = bump.vault;
        self.vault_state.state_bump = bump.vault_state;
        Ok(())
    }
}


#[derive(Accounts)]
pub struct CloseAccount<'info>{
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds =[b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
        close = user
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CloseAccount<'info>{
    pub fn close(&mut self)-> Result<()>{
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer{
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let user_key = self.user.key();
            let seeds = &[
                b"vault",
                user_key.as_ref(),
                &[self.vault_state.vault_bump],
            ];
        
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program,cpi_accounts,signer_seeds);

        transfer(cpi_ctx, self.vault.lamports())
    }
}
#[derive(Accounts)]
pub struct Payment<'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds =[b"state", user.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
impl<'info> Payment<'info>{
    pub fn deposit(&mut self, amount: u64) -> Result<()>{
        
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer{
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program,cpi_accounts);
        transfer(cpi_ctx, amount)
        
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()>{
        
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer{
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };
        let user_key = self.user.key(); //store in variable to extend lifetime, avoids "temporary value dropped" 
        let seeds = &[
            b"vault",
            user_key.as_ref(),
            &[self.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program,cpi_accounts,signer_seeds);
        transfer(cpi_ctx, amount)
        
    }
    
}

#[account]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl Space for VaultState {

    const INIT_SPACE: usize = 8 + 1 + 1;    
}