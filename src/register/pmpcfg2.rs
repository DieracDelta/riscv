use register::PmpAField;
// only meant for 32 bit
pub struct Pmpcfg2 {
    bits: usize,
}

impl Pmpcfg2 {
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    // r0 field
    #[inline]
    pub fn get_r8(&self) -> bool {
        self.bits & (1 << 0) == 1 << 0
    }

    // w0 field
    #[inline]
    pub fn get_w8(&self) -> bool {
        self.bits & (1 << 1) == 1 << 1
    }

    // x0 field
    #[inline]
    pub fn get_x8(&self) -> bool {
        self.bits & (1 << 2) == 1 << 2
    }

    // a0 field
    #[inline]
    pub fn get_a8(&self) -> PmpAField {
        match (self.bits & (0b11 << 3)) >> 3 {
            0b00 => PmpAField::OFF,
            0b01 => PmpAField::TOR,
            0b10 => PmpAField::NA4,
            0b11 => PmpAField::NAPOT,
            // should never hit this case
            _ => PmpAField::OFF,
        }
    }

    // l0 field
    #[inline]
    pub fn get_l8(&self) -> bool {
        self.bits & (1 << 7) == 1 << 7
    }

    // r1 field
    #[inline]
    pub fn get_r9(&self) -> bool {
        self.bits & (1 << 8) == 1 << 8
    }

    // w1 field
    #[inline]
    pub fn get_w9(&self) -> bool {
        self.bits & (1 << 9) == 1 << 9
    }

    // x1 field
    #[inline]
    pub fn get_x9(&self) -> bool {
        self.bits & (1 << 10) == 1 << 10
    }

    // a1 field
    #[inline]
    pub fn get_a9(&self) -> PmpAField {
        match (self.bits & (0b11 << 11)) >> 11 {
            0b00 => PmpAField::OFF,
            0b01 => PmpAField::TOR,
            0b10 => PmpAField::NA4,
            0b11 => PmpAField::NAPOT,
            // should never hit this case
            _ => PmpAField::OFF,
        }
    }

    // l1 field
    #[inline]
    pub fn get_l9(&self) -> bool {
        self.bits & (1 << 15) == 1 << 15
    }

    // r2 field
    #[inline]
    pub fn get_r10(&self) -> bool {
        self.bits & (1 << 16) == 1 << 16
    }

    // w2 field
    #[inline]
    pub fn get_w10(&self) -> bool {
        self.bits & (1 << 17) == 1 << 17
    }

    // x2 field
    #[inline]
    pub fn get_x10(&self) -> bool {
        self.bits & (1 << 18) == 1 << 18
    }

    // a2 field
    #[inline]
    pub fn get_a10(&self) -> PmpAField {
        match (self.bits & (0b11 << 19)) >> 19 {
            0b00 => PmpAField::OFF,
            0b01 => PmpAField::TOR,
            0b10 => PmpAField::NA4,
            0b11 => PmpAField::NAPOT,
            // should never hit this case
            _ => PmpAField::OFF,
        }
    }

    // l2 field
    #[inline]
    pub fn get_l10(&self) -> bool {
        self.bits & (1 << 23) == 1 << 23
    }

    // r3 field
    #[inline]
    pub fn get_r11(&self) -> bool {
        self.bits & (1 << 24) == 1 << 24
    }

    // w3 field
    #[inline]
    pub fn get_w11(&self) -> bool {
        self.bits & (1 << 25) == 1 << 25
    }

    // x3 field
    #[inline]
    pub fn get_x11(&self) -> bool {
        self.bits & (1 << 26) == 1 << 26
    }

    // a3 field
    #[inline]
    pub fn get_a11(&self) -> PmpAField {
        match (self.bits & (0b11 << 27)) >> 27 {
            0b00 => PmpAField::OFF,
            0b01 => PmpAField::TOR,
            0b10 => PmpAField::NA4,
            0b11 => PmpAField::NAPOT,
            // should never hit this case
            _ => PmpAField::OFF,
        }
    }

    // l3 field
    #[inline]
    pub fn get_l11(&self) -> bool {
        self.bits & (1 << 31) == 1 << 31
    }
}

read_csr_as!(Pmpcfg2, 0x3A2, __read_pmpcfg2);
set!(0x3A2, __set_pmpcfg2);
clear!(0x3A2, __clear_pmpcfg2);

set_clear_csr!(
    /// r0
    , set_r8, clear_r8, 1 << 0);
set_clear_csr!(
    /// w0
    , set_w8, clear_w8, 1 << 1);
set_clear_csr!(
    /// x0
    , set_x8, clear_x8, 1 << 2);
set_clear_csr!(
    /// l0
    , set_l8, clear_l8, 1 << 7);
set_clear_csr!(
    /// r1
    , set_r9, clear_r9, 1 << 8);
set_clear_csr!(
    /// w1
    , set_w9, clear_w9, 1 << 9);
set_clear_csr!(
    /// x1
    , set_x9, clear_x9, 1 << 10);
set_clear_csr!(
    /// l1
    , set_l9, clear_l9, 1 << 15);
set_clear_csr!(
    /// r2
    , set_r10, clear_r10, 1 << 16);
set_clear_csr!(
    /// w2
    , set_w10, clear_w10, 1 << 17);
set_clear_csr!(
    /// x2
    , set_x10, clear_x10, 1 << 18);
set_clear_csr!(
    /// l2
    , set_l10, clear_l10, 1 << 23);
set_clear_csr!(
    /// r3
    , set_r11, clear_r11, 1 << 24);
set_clear_csr!(
    /// w3
    , set_w11, clear_w11, 1 << 25);
set_clear_csr!(
    /// x3
    , set_x11, clear_x11, 1 << 26);
set_clear_csr!(
    /// l3
    , set_l11, clear_l11, 1 << 31);

#[inline]
pub unsafe fn set_a8(a_field: PmpAField) {
    _set((a_field as usize) << 3);
}
#[inline]
pub unsafe fn set_a9(a_field: PmpAField) {
    _set((a_field as usize) << 11);
}
#[inline]
pub unsafe fn set_a10(a_field: PmpAField) {
    _set((a_field as usize) << 19);
}
#[inline]
pub unsafe fn set_a11(a_field: PmpAField) {
    _set((a_field as usize) << 27);
}
