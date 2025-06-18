use pinocchio::program_error::ProgramError;

pub enum PinocchioError {
    NotSigner,
    InvalidOwner,
    InvalidAccountData,
    InvalidCoordinate,
    InvalidColorComponent,
}

impl From<PinocchioError> for ProgramError {
    fn from(e: PinocchioError) -> Self {
        Self::Custom(e as u32)
    }
}