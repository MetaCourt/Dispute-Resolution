use anchor_lang::prelude::*;

declare_id!("8rLpdGuKpqPRYF9odc1ken4AnxQfTF1tiXUTM2zJDXQ1");

#[program]
pub mod dispute_resolution {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
