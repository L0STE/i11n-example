use anchor_lang::prelude::*;

declare_id!("J8gQu1WUpZGpJNCBM5JjrSe4DxxJXRWXq6GFN5RpDYsz");

// Accounts
#[derive(Accounts)]
pub struct Make<'info> {
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
}

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut, signer)]
    /// CHECK: the CPI will check it for us
    pub maker: AccountInfo<'info>,
    /// CHECK: the CPI will check it for us
    pub mint_a: AccountInfo<'info>,
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
}

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut, signer)]
    /// CHECK: the CPI will check it for us

    pub taker: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: the CPI will check it for us
    pub maker: AccountInfo<'info>,
    /// CHECK: the CPI will check it for us
    pub mint_a: AccountInfo<'info>,
    /// CHECK: the CPI will check it for us
    pub mint_b: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: the CPI will check it for us
    pub taker_ata_a: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: the CPI will check it for us
    pub taker_ata_b: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: the CPI will check it for us
    pub maker_ata_b: AccountInfo<'info>,
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
}

// CPI
pub mod cpi {
    #![allow(unused)]
    use super::*;

    pub fn make<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, Make<'info>>,
        seed: u64,
        deposit: u64,
        receive: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::Make { seed, deposit, receive };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&[138,227,232,77,223,166,96,197]);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn refund<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, Refund<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::Refund {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&[2,96,183,251,63,208,46,46]);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn take<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, Take<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::Take {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&[149,226,52,104,6,142,230,39]);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }  
}

// I11n
pub mod i11n {
    use anchor_lang::prelude::*;
    use anchor_i11n::prelude::*;
    use super::{instructions::*, ID};

    // Make
    #[derive(TryFromInstruction)]
    pub struct MakeI11n<'info> {
        pub accounts: MakeAccountMetas<'info>,
        pub args: Make
    }

    // Refund
    #[derive(TryFromInstruction)]
    pub struct RefundI11n<'info> {
        pub accounts: RefundAccountMetas<'info>,
        pub args: Refund
    }

    // Take
    #[derive(TryFromInstruction)]
    pub struct TakeI11n<'info> {
        pub accounts: TakeAccountMetas<'info>,
        pub args: Take
    }

    //Accounts
    #[derive(TryFromAccountMetas)]
    pub struct MakeAccountMetas<'info> {
        pub maker: &'info AccountMeta,
        pub mint_a: &'info AccountMeta,
        pub mint_b: &'info AccountMeta,
        pub maker_ata_a: &'info AccountMeta,
        pub escrow: &'info AccountMeta,
        pub vault: &'info AccountMeta,
        pub associated_token_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct RefundAccountMetas<'info> {
        pub maker: &'info AccountMeta,
        pub mint_a: &'info AccountMeta,
        pub maker_ata_a: &'info AccountMeta,
        pub escrow: &'info AccountMeta,
        pub vault: &'info AccountMeta,
        pub associated_token_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct TakeAccountMetas<'info> {
        pub taker: &'info AccountMeta,
        pub maker: &'info AccountMeta,
        pub mint_a: &'info AccountMeta,
        pub mint_b: &'info AccountMeta,
        pub taker_ata_a: &'info AccountMeta,
        pub taker_ata_b: &'info AccountMeta,
        pub maker_ata_b: &'info AccountMeta,
        pub escrow: &'info AccountMeta,
        pub vault: &'info AccountMeta,
        pub associated_token_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }
}

// Instructions
pub mod instructions {
    use super::*;

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct Make {
        pub seed: u64,
        pub deposit: u64,
        pub receive: u64,
    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct Refund {

    }

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct Take {

    }        
}
        
// Defined types
