#![deny(clippy::all)]

use anchor_lang::prelude::*;

declare_id!("Bp1B4Azj7AVjvg855Xd7AiPrJFpSxMZMqRSbpUcXaSRe");

#[program]
pub mod entros_token {
    use super::*;

    /// Initialize the Entros token mint.
    /// Implementation in Phase 7.
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
