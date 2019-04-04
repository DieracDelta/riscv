//! mie register

use bit_field::BitField;

/// mie register
#[derive(Clone, Copy, Debug)]
pub struct Mie {
    bits: usize,
}

impl Mie {
    /// Returns the contents of the register as raw bits
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// User Software Interrupt Enable
    #[inline]
    pub fn usoft(&self) -> bool {
        self.bits.get_bit(0)
    }

    /// Supervisor Software Interrupt Enable
    #[inline]
    pub fn ssoft(&self) -> bool {
        self.bits.get_bit(1)
    }

    /// Machine Software Interrupt Enable
    #[inline]
    pub fn msoft(&self) -> bool {
        self.bits.get_bit(3)
    }

    /// User Timer Interrupt Enable
    #[inline]
    pub fn utimer(&self) -> bool {
        self.bits.get_bit(4)
    }

    /// Supervisor Timer Interrupt Enable
    #[inline]
    pub fn stimer(&self) -> bool {
        self.bits.get_bit(5)
    }

    /// Machine Timer Interrupt Enable
    #[inline]
    pub fn mtimer(&self) -> bool {
        self.bits.get_bit(7)
    }

    /// User External Interrupt Enable
    #[inline]
    pub fn uext(&self) -> bool {
        self.bits.get_bit(8)
    }

    /// Supervisor External Interrupt Enable
    #[inline]
    pub fn sext(&self) -> bool {
        self.bits.get_bit(9)
    }

    /// Machine External Interrupt Enable
    #[inline]
    pub fn mext(&self) -> bool {
        self.bits.get_bit(11)
    }

    /// local interrupt enable bit 0
    #[inline]
    pub fn lie0(&self) -> bool {
        self.bits.get_bit(16)
    }

    /// local interrupt enable bit 1
    #[inline]
    pub fn lie1(&self) -> bool {
        self.bits.get_bit(17)
    }

    /// local interrupt enable bit 2
    #[inline]
    pub fn lie2(&self) -> bool {
        self.bits.get_bit(18)
    }

    /// local interrupt enable bit 3
    #[inline]
    pub fn lie3(&self) -> bool {
        self.bits.get_bit(19)
    }

    /// local interrupt enable bit 4
    #[inline]
    pub fn lie4(&self) -> bool {
        self.bits.get_bit(20)
    }

    /// local interrupt enable bit 5
    #[inline]
    pub fn lie5(&self) -> bool {
        self.bits.get_bit(21)
    }

    /// local interrupt enable bit 6
    #[inline]
    pub fn lie6(&self) -> bool {
        self.bits.get_bit(22)
    }

    /// local interrupt enable bit 7
    #[inline]
    pub fn lie7(&self) -> bool {
        self.bits.get_bit(23)
    }

    /// local interrupt enable bit 8
    #[inline]
    pub fn lie8(&self) -> bool {
        self.bits.get_bit(24)
    }

    /// local interrupt enable bit 9
    #[inline]
    pub fn lie9(&self) -> bool {
        self.bits.get_bit(25)
    }

    /// local interrupt enable bit 10
    #[inline]
    pub fn lie10(&self) -> bool {
        self.bits.get_bit(26)
    }

    /// local interrupt enable bit 11
    #[inline]
    pub fn lie11(&self) -> bool {
        self.bits.get_bit(27)
    }

    /// local interrupt enable bit 12
    #[inline]
    pub fn lie12(&self) -> bool {
        self.bits.get_bit(28)
    }

    /// local interrupt enable bit 13
    #[inline]
    pub fn lie13(&self) -> bool {
        self.bits.get_bit(29)
    }

    /// local interrupt enable bit 14
    #[inline]
    pub fn lie14(&self) -> bool {
        self.bits.get_bit(30)
    }

    /// local interrupt enable bit 15
    #[inline]
    pub fn lie15(&self) -> bool {
        self.bits.get_bit(31)
    }
}

read_csr_as!(Mie, 0x304, __read_mie);
set!(0x304, __set_mie);
clear!(0x304, __clear_mie);

set_clear_csr!(
    /// User Software Interrupt Enable
    , set_usoft, clear_usoft, 1 << 0);
set_clear_csr!(
    /// Supervisor Software Interrupt Enable
    , set_ssoft, clear_ssoft, 1 << 1);
set_clear_csr!(
    /// Machine Software Interrupt Enable
    , set_msoft, clear_msoft, 1 << 3);
set_clear_csr!(
    /// User Timer Interrupt Enable
    , set_utimer, clear_utimer, 1 << 4);
set_clear_csr!(
    /// Supervisor Timer Interrupt Enable
    , set_stimer, clear_stimer, 1 << 5);
set_clear_csr!(
    /// Machine Timer Interrupt Enable
    , set_mtimer, clear_mtimer, 1 << 7);
set_clear_csr!(
    /// User External Interrupt Enable
    , set_uext, clear_uext, 1 << 8);
set_clear_csr!(
    /// Supervisor External Interrupt Enable
    , set_sext, clear_sext, 1 << 9);
set_clear_csr!(
    /// Machine External Interrupt Enable
    , set_mext, clear_mext, 1 << 11);
set_clear_csr!(
    /// Machine Local Interrupt 0 Enable
    , set_lie0, clear_lie0, 1 << 16);
set_clear_csr!(
    /// Machine Local Interrupt 1 Enable
    , set_lie1, clear_lie1, 1 << 17);
set_clear_csr!(
    /// Machine Local Interrupt 2 Enable
    , set_lie2, clear_lie2, 1 << 18);
set_clear_csr!(
    /// Machine Local Interrupt 3 Enable
    , set_lie3, clear_lie3, 1 << 19);
set_clear_csr!(
    /// Machine Local Interrupt 4 Enable
    , set_lie4, clear_lie4, 1 << 20);
set_clear_csr!(
    /// Machine Local Interrupt 5 Enable
    , set_lie5, clear_lie5, 1 << 21);
set_clear_csr!(
    /// Machine Local Interrupt 6 Enable
    , set_lie6, clear_lie6, 1 << 22);
set_clear_csr!(
    /// Machine Local Interrupt 7 Enable
    , set_lie7, clear_lie7, 1 << 23);
set_clear_csr!(
    /// Machine Local Interrupt 8 Enable
    , set_lie8, clear_lie8, 1 << 24);
set_clear_csr!(
    /// Machine Local Interrupt 9 Enable
    , set_lie9, clear_lie9, 1 << 25);
set_clear_csr!(
    /// Machine Local Interrupt 10 Enable
    , set_lie10, clear_lie10, 1 << 26);
set_clear_csr!(
    /// Machine Local Interrupt 11 Enable
    , set_lie11, clear_lie11, 1 << 27);
set_clear_csr!(
    /// Machine Local Interrupt 12 Enable
    , set_lie12, clear_lie12, 1 << 28);
set_clear_csr!(
    /// Machine Local Interrupt 13 Enable
    , set_lie13, clear_lie13, 1 << 29);
set_clear_csr!(
    /// Machine Local Interrupt 14 Enable
    , set_lie14, clear_lie14, 1 << 30);
set_clear_csr!(
    /// Machine Local Interrupt 15 Enable
    , set_lie15, clear_lie15, 1 << 31);
