use anchor_lang::prelude::*;

declare_id!("GyjkaXHRBEAxJz3iyXCE5ux63q3vZTSCKikp1tfJbuFT");

#[program]
pub mod basic {
    use super::*;

    pub fn greet(_ctx: Context<Initialize>) -> Result<()> {
        msg!("GM!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
