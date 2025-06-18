use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::create_program_address,
    ProgramResult,
};

use crate::{
    AccountCheck, ColorData, Pixel, ProgramAccount, SignerAccount,
};
use core::mem::size_of;

pub struct UpdateAccounts<'a> {
    pub pixel: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for UpdateAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [user, pixel, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // Basic Accounts Checks
        SignerAccount::check(user)?;
        ProgramAccount::check(pixel)?;

        // Return the accounts
        Ok(Self { pixel })
    }
}

pub struct UpdateInstructionData {
    pub color_r: u8,
    pub color_g: u8,
    pub color_b: u8,
}

impl<'a> TryFrom<&'a [u8]> for UpdateInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        if data.len() != size_of::<u8>() * 3 {
            return Err(ProgramError::InvalidInstructionData);
        }

        let color_r = u8::from_le_bytes(data[0..1].try_into().unwrap());
        let color_g = u8::from_le_bytes(data[1..2].try_into().unwrap());
        let color_b = u8::from_le_bytes(data[2..3].try_into().unwrap());

        // Instruction Checks
        ColorData::check(color_r)?;
        ColorData::check(color_g)?;
        ColorData::check(color_b)?;

        Ok(Self {
            color_r,
            color_g,
            color_b,
        })
    }
}

pub struct Update<'a> {
    pub accounts: UpdateAccounts<'a>,
    pub instruction_data: UpdateInstructionData,
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Update<'a> {
    type Error = ProgramError;

    fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
        let accounts = UpdateAccounts::try_from(accounts)?;
        let instruction_data = UpdateInstructionData::try_from(data)?;

        Ok(Self {
            accounts,
            instruction_data,
        })
    }
}

impl<'a> Update<'a> {
    pub const DISCRIMINATOR: &'a u8 = &1;

    pub fn process(&mut self) -> ProgramResult {
        // Populate the pixel account
        let mut data = self.accounts.pixel.try_borrow_mut_data()?;
        let pixel = Pixel::load_mut(data.as_mut())?;

        // Check if the pixel is valid
        let pixel_key = create_program_address(
            &[
                b"pixel",
                &pixel.pos_x.to_le_bytes(),
                &pixel.pos_y.to_le_bytes(),
                &pixel.bump,
            ],
            &crate::ID,
        )?;
        if &pixel_key != self.accounts.pixel.key() {
            return Err(ProgramError::InvalidAccountOwner);
        }

        pixel.set_color_r(self.instruction_data.color_r);
        pixel.set_color_g(self.instruction_data.color_g);
        pixel.set_color_b(self.instruction_data.color_b);

        Ok(())
    }
}
