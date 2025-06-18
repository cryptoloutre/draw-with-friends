use anchor_lang::prelude::*;

mod errors;
mod instructions;
mod state;
use instructions::*;

declare_id!("H9dnZP25nrp4HVLZvfQCvQCmaqMwJbv8qmfLjv2Uzhnr");

#[program]
pub mod draw_with_friends_anchor {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn create(
        ctx: Context<Create>,
        pos_x: u8,
        pos_y: u8,
        col_r: u8,
        col_g: u8,
        col_b: u8,
    ) -> Result<()> {
        instructions::create::handler(ctx, pos_x, pos_y, col_r, col_g, col_b)
    }

    #[instruction(discriminator = 1)]
    pub fn update(ctx: Context<Update>, col_r: u8, col_g: u8, col_b: u8) -> Result<()> {
        instructions::update::handler(ctx, col_r, col_g, col_b)
    }
}
