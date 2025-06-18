use core::mem::size_of;
use pinocchio::program_error::ProgramError;

#[repr(C)]
pub struct Pixel {
    pub pos_x: u8,     // x coordinate of the pixel
    pub pos_y: u8,     // y coordinate of the pixel
    pub color_r: u8,   // red component
    pub color_g: u8,   // green component
    pub color_b: u8,   // blue component
    pub bump: [u8; 1], // PDA bump seed
}

impl Pixel {
    pub const LEN: usize = size_of::<u8>()
        + size_of::<u8>()
        + size_of::<u8>()
        + size_of::<u8>()
        + size_of::<u8>()
        + size_of::<[u8; 1]>();

    #[inline(always)]
    pub fn load_mut(bytes: &mut [u8]) -> Result<&mut Self, ProgramError> {
        if bytes.len() != Pixel::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(unsafe { &mut *core::mem::transmute::<*mut u8, *mut Self>(bytes.as_mut_ptr()) })
    }

    // #[inline(always)]
    // pub fn load(bytes: &[u8]) -> Result<&Self, ProgramError> {
    //     if bytes.len() != Pixel::LEN {
    //         return Err(ProgramError::InvalidAccountData);
    //     }
    //     Ok(unsafe { &*core::mem::transmute::<*const u8, *const Self>(bytes.as_ptr()) })
    // }

    #[inline(always)]
    pub fn set_pox_x(&mut self, pos_x: u8) {
        self.pos_x = pos_x;
    }

    #[inline(always)]
    pub fn set_pox_y(&mut self, pos_y: u8) {
        self.pos_y = pos_y;
    }

    #[inline(always)]
    pub fn set_color_r(&mut self, color_r: u8) {
        self.color_r = color_r;
    }

    #[inline(always)]
    pub fn set_color_g(&mut self, color_g: u8) {
        self.color_g = color_g;
    }

    #[inline(always)]
    pub fn set_color_b(&mut self, color_b: u8) {
        self.color_b = color_b;
    }

    // #[inline(always)]
    // pub fn set_bump(&mut self, bump: [u8; 1]) {
    //     self.bump = bump;
    // }

    #[inline(always)]
    pub fn set_inner(
        &mut self,
        pos_x: u8,
        pos_y: u8,
        color_r: u8,
        color_g: u8,
        color_b: u8,
        bump: [u8; 1],
    ) {
        self.pos_x = pos_x;
        self.pos_y = pos_y;
        self.color_r = color_r;
        self.color_g = color_g;
        self.color_b = color_b;
        self.bump = bump;
    }
}
