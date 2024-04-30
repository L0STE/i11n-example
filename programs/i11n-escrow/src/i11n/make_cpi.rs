use anchor_lang::prelude::*;
use crate::macros::{cpi::make, Make};

#[derive(Accounts)]
pub struct MakeCpi<'info> {
    #[account(mut, signer)]
    /// CHECK: the CPI will check it for us
    pub maker: AccountInfo<'info>,
    /// CHECK: the CPI will check it for us
    pub mint_a: AccountInfo<'info>,
    /// CHECK: the CPI will check it for us
    pub mint_b: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: the CPI will check it for us
    pub maker_ata_a: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: the CPI will check it for us
    pub escrow: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: the CPI will check it for us
    pub vault: AccountInfo<'info>,
    /// CHECK: the CPI will check it for us
    pub associated_token_program: AccountInfo<'info>,
    /// CHECK: the CPI will check it for us
    pub token_program: AccountInfo<'info>,
    /// CHECK: the CPI will check it for us
    pub system_program: AccountInfo<'info>,
    /// CHECK: the CPI will check it for us
    pub escrow_program: AccountInfo<'info>,
}

impl<'info> MakeCpi<'info> {
    pub fn make_cpi(&mut self, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        let ctx = CpiContext::new(
            self.escrow_program.to_account_info(),
            Make {
                maker: self.maker.to_account_info(),
                mint_a: self.mint_a.to_account_info(),
                mint_b: self.mint_a.to_account_info(),
                maker_ata_a: self.maker_ata_a.to_account_info(),
                escrow: self.escrow.to_account_info(),
                vault: self.vault.to_account_info(),
                associated_token_program: self.associated_token_program.to_account_info(),
                token_program: self.token_program.to_account_info(),
                system_program: self.system_program.to_account_info(),
            }
        );

        make(ctx, seed, deposit, receive)
    }
}