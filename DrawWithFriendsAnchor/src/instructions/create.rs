use crate::errors::MyError;
use crate::state::Pixel;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(pos_x: u8, pos_y: u8)]
pub struct Create<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
    init,
    payer = user,
    space = Pixel::INIT_SPACE + Pixel::DISCRIMINATOR.len(),
    seeds = [b"pixel", pos_x.to_le_bytes().as_ref(), pos_y.to_le_bytes().as_ref()],
    bump,
  )]
    pub pixel: Account<'info, Pixel>,
    pub system_program: Program<'info, System>,
}

impl<'info> Create<'info> {
    // Create the Pixel
    fn populate_pixel(
        &mut self,
        pos_x: u8,
        pos_y: u8,
        color_r: u8,
        color_g: u8,
        color_b: u8,
        bump: u8,
    ) -> Result<()> {
        self.pixel.set_inner(Pixel {
            pos_x,
            pos_y,
            color_r,
            color_g,
            color_b,
            bump,
        });

        Ok(())
    }
}

const MIN_POS: u8 = 0;
const MAX_POS: u8 = 99;
const MIN_COL: u8 = 0;
const MAX_COL: u8 = 255;

pub fn handler(
    ctx: Context<Create>,
    pos_x: u8,
    pos_y: u8,
    color_r: u8,
    color_g: u8,
    color_b: u8,
) -> Result<()> {
    // Validate the data
    if pos_x < MIN_POS || pos_x > MAX_POS {
        return Err(MyError::InvalidCoordinate.into());
    }
    if pos_y < MIN_POS || pos_y > MAX_POS {
        return Err(MyError::InvalidCoordinate.into());
    }
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
    ctx.accounts
        .populate_pixel(pos_x, pos_y, color_r, color_g, color_b, ctx.bumps.pixel)?;

    Ok(())
}
