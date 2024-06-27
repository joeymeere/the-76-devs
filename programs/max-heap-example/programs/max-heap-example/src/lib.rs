use anchor_lang::prelude::*;

declare_id!("DCYPUmuJrDrP6JMLbxQPoH28rguNFnAou7pMMyj51L3S");

#[program]
pub mod max_heap_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
