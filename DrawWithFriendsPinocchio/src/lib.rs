#![no_std]
use pinocchio::{
    account_info::AccountInfo, entrypoint, nostd_panic_handler, program_error::ProgramError, pubkey::Pubkey, ProgramResult
};
entrypoint!(process_instruction);
nostd_panic_handler!();

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

pub mod errors;
pub use errors::*;

pub const ID: Pubkey = [
    0xa9, 0xa7, 0x0A, 0x27, 0x05, 0x3A, 0x61, 0x9E, 0x9D, 0x99, 0x7F, 0x20, 0xDA, 0x29, 0x22, 0xE3, 0xD8, 0x44, 0xF8, 0x80, 0x81, 0xD4, 0x7E, 0x1B, 0xFD, 0xE2, 0x45, 0xCB, 0x95, 0x73, 0xD8, 0x47,
];

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((Create::DISCRIMINATOR, data)) => Create::try_from((data, accounts))?.process(),
        Some((Update::DISCRIMINATOR, data)) => Update::try_from((data, accounts))?.process(),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}