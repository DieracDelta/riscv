//! satp register

use bit_field::BitField;

/// satp register
#[derive(Clone, Copy, Debug)]
pub struct Satp {
    bits: usize,
}

impl Satp {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Current address-translation scheme
    #[inline]
    pub fn mode(&self) -> Mode {
        match self.bits.get_bit(31) {
            false => Mode::Bare,
            true => Mode::Sv32,
        }
    }

    /// Address space identifier
    #[inline]
    pub fn asid(&self) -> usize {
        self.bits.get_bits(22..31)
    }

    /// Physical page number
    #[inline]
    pub fn ppn(&self) -> usize {
        self.bits.get_bits(0..22)
    }

}

pub enum Mode {
    Bare = 0,
    Sv32 = 1,
}

read_csr_as!(Satp, 0x180, __read_satp);
write_csr!(0x180, __write_satp);

#[inline]
pub unsafe fn set(mode: Mode, asid: usize, ppn: usize) {
    let mut bits = 0usize;
    bits.set_bits(31..32, mode as usize);
    bits.set_bits(22..31, asid);
    bits.set_bits(0..22, ppn);
    _write(bits);
}
