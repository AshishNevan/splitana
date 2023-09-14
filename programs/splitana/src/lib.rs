use anchor_lang::prelude::*;

declare_id!("73qPic8NHyrWJQdX16PSEqESYBKZo4y7awKUYF3kTszY");

#[program]
pub mod splitana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
