use anchor_lang::prelude::*;
 
#[error_code]
pub enum MyError {
  #[msg("The given co-ordinate is not between 0-99")]
  InvalidCoordinate,
  #[msg("A given color component is not between 0-255")]
  InvalidColorComponent,
}