use anchor_lang::prelude::*;
use solana_program::sysvar::instructions::{
    self,
    load_current_index_checked, 
    load_instruction_at_checked
};
use anchor_escrow_sdk::i11n::MakeI11n;

#[derive(Accounts)]
pub struct IntrospectionCheck<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(address = instructions::ID)]
    /// CHECK: InstructionsSysvar account
    instructions: UncheckedAccount<'info>,
}

impl<'info> IntrospectionCheck<'info> {
    pub fn maker_check(&self) -> Result<()> {
        let index = load_current_index_checked(&self.instructions.to_account_info())?;
        let ix = load_instruction_at_checked(index as usize - 1, &self.instructions.to_account_info())?;

        let instruction = MakeI11n::try_from(&ix)?;
        msg!("MakeArgs: seed: {:?}", instruction.args.seed);
        msg!("MakeAccounts: maker: {:?}", instruction.accounts.maker);

        Ok(())
    }
}