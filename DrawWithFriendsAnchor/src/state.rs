use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account(discriminator = 1)]
pub struct Pixel {
    pub pos_x: u8,   // x coordinate of the pixel
    pub pos_y: u8,   // y coordinate of the pixel
    pub color_r: u8, // red component
    pub color_g: u8, // green component
    pub color_b: u8, // blue component
    pub bump: u8, // PDA bump seed
}
