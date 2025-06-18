use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed},
    program_error::ProgramError,
    pubkey::find_program_address,
    ProgramResult,
};

use crate::{ColorData, Pixel, CoordinateData, ProgramAccount, ProgramAccountInit, SignerAccount};
use core::mem::size_of;

pub struct CreateAccounts<'a> {
    pub user: &'a AccountInfo,
    pub pixel: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for CreateAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [user, pixel, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // Basic Accounts Checks
        SignerAccount::check(user)?;

        // Return the accounts
        Ok(Self { user, pixel })
    }
}

pub struct CreateInstructionData {
    pub pos_x: u8,
    pub pos_y: u8,
    pub color_r: u8,
    pub color_g: u8,
    pub color_b: u8,
}

impl<'a> TryFrom<&'a [u8]> for CreateInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != size_of::<u8>() * 5 {
            return Err(ProgramError::InvalidInstructionData);
        }

        let pos_x = u8::from_le_bytes(data[0..1].try_into().unwrap());
        let pos_y = u8::from_le_bytes(data[1..2].try_into().unwrap());
        let color_r = u8::from_le_bytes(data[2..3].try_into().unwrap());
        let color_g = u8::from_le_bytes(data[3..4].try_into().unwrap());
        let color_b = u8::from_le_bytes(data[4..5].try_into().unwrap());

        // Instruction Checks
        CoordinateData::check(pos_x)?;
        CoordinateData::check(pos_y)?;
        ColorData::check(color_r)?;
        ColorData::check(color_g)?;
        ColorData::check(color_b)?;

        Ok(Self {
            pos_x,
            pos_y,
            color_r,
            color_g,
            color_b,
        })
    }
}

pub struct Create<'a> {
    pub accounts: CreateAccounts<'a>,
    pub instruction_data: CreateInstructionData,
    pub bump: u8,
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Create<'a> {
    type Error = ProgramError;

    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = CreateAccounts::try_from(accounts)?;
        let instruction_data = CreateInstructionData::try_from(data)?;

        // Initialize the Accounts needed
        let (_, bump) = find_program_address(
            &[
                b"pixel",
                &instruction_data.pos_x.to_le_bytes(),
                &instruction_data.pos_y.to_le_bytes(),
            ],
            &crate::ID,
        );

        let pos_x_binding = instruction_data.pos_x.to_le_bytes();
        let pos_y_binding = instruction_data.pos_y.to_le_bytes();
        let bump_binding = [bump];
        let pixel_seeds = [
            Seed::from(b"pixel"),
            Seed::from(&pos_x_binding),
            Seed::from(&pos_y_binding),
            Seed::from(&bump_binding),
        ];

        ProgramAccount::init::<Pixel>(accounts.user, accounts.pixel, &pixel_seeds, Pixel::LEN)?;

        Ok(Self {
            accounts,
            instruction_data,
            bump,
        })
    }
}

impl<'a> Create<'a> {
    pub const DISCRIMINATOR: &'a u8 = &0;

    pub fn process(&mut self) -> ProgramResult {
        // Populate the pixel account
        let mut data = self.accounts.pixel.try_borrow_mut_data()?;
        let pixel = Pixel::load_mut(data.as_mut())?;

        pixel.set_inner(
            self.instruction_data.pos_x,
            self.instruction_data.pos_y,
            self.instruction_data.color_r,
            self.instruction_data.color_g,
            self.instruction_data.color_b,
            [self.bump],
        );

        Ok(())
    }
}
