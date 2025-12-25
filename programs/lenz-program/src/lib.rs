use anchor_lang::prelude::*;
declare_id!("Audit11111111111111111111111111111111111111");

#[program]
pub mod lenz_program {
    use super::*;
    pub fn register_scan(_ctx: Context<RegisterScan>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterScan {}