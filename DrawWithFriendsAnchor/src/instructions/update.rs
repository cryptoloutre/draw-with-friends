use crate::errors::MyError;
use crate::state::Pixel;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
    mut,
    seeds = [b"pixel", pixel.pos_x.to_le_bytes().as_ref(), pixel.pos_y.to_le_bytes().as_ref()],
    bump = pixel.bump,
  )]
    pub pixel: Account<'info, Pixel>,
    pub system_program: Program<'info, System>,
}

impl<'info> Update<'info> {
    // Update the Pixel
    fn populate_pixel(&mut self, color_r: u8, color_g: u8, color_b: u8) -> Result<()> {
        self.pixel.color_r = color_r;
        self.pixel.color_g = color_g;
        self.pixel.color_b = color_b;

        Ok(())
    }
}

const MIN_COL: u8 = 0;
const MAX_COL: u8 = 255;

pub fn handler(ctx: Context<Update>, color_r: u8, color_g: u8, color_b: u8) -> Result<()> {
    // Validate the data
    if color_r < MIN_COL || color_r > MAX_COL {
        return Err(MyError::InvalidColorComponent.into());
    }
    if color_g < MIN_COL || color_g > MAX_COL {
        return Err(MyError::InvalidColorComponent.into());
    }
    if color_b < MIN_COL || color_b > MAX_COL {
        return Err(MyError::InvalidColorComponent.into());
    }

    // Save the Pixel Data
    ctx.accounts.populate_pixel(color_r, color_g, color_b)?;

    Ok(())
}
