use register::PmpAField;
// only meant for 32 bit
pub struct Pmpcfg3 {
    bits: usize,
}

impl Pmpcfg3 {
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    // r0 field
    #[inline]
    pub fn get_r12(&self) -> bool {
        self.bits & (1 << 0) == 1 << 0
    }

    // w0 field
    #[inline]
    pub fn get_w12(&self) -> bool {
        self.bits & (1 << 1) == 1 << 1
    }

    // x0 field
    #[inline]
    pub fn get_x12(&self) -> bool {
        self.bits & (1 << 2) == 1 << 2
    }

    // a0 field
    #[inline]
    pub fn get_a12(&self) -> PmpAField {
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
    pub fn get_l12(&self) -> bool {
        self.bits & (1 << 7) == 1 << 7
    }

    // r1 field
    #[inline]
    pub fn get_r13(&self) -> bool {
        self.bits & (1 << 8) == 1 << 8
    }

    // w1 field
    #[inline]
    pub fn get_w13(&self) -> bool {
        self.bits & (1 << 9) == 1 << 9
    }

    // x1 field
    #[inline]
    pub fn get_x13(&self) -> bool {
        self.bits & (1 << 10) == 1 << 10
    }

    // a1 field
    #[inline]
    pub fn get_a13(&self) -> PmpAField {
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
    pub fn get_l13(&self) -> bool {
        self.bits & (1 << 15) == 1 << 15
    }

    // r2 field
    #[inline]
    pub fn get_r14(&self) -> bool {
        self.bits & (1 << 16) == 1 << 16
    }

    // w2 field
    #[inline]
    pub fn get_w14(&self) -> bool {
        self.bits & (1 << 17) == 1 << 17
    }

    // x2 field
    #[inline]
    pub fn get_x14(&self) -> bool {
        self.bits & (1 << 18) == 1 << 18
    }

    // a2 field
    #[inline]
    pub fn get_a14(&self) -> PmpAField {
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
    pub fn get_l14(&self) -> bool {
        self.bits & (1 << 23) == 1 << 23
    }

    // r3 field
    #[inline]
    pub fn get_r15(&self) -> bool {
        self.bits & (1 << 24) == 1 << 24
    }

    // w3 field
    #[inline]
    pub fn get_w15(&self) -> bool {
        self.bits & (1 << 25) == 1 << 25
    }

    // x3 field
    #[inline]
    pub fn get_x15(&self) -> bool {
        self.bits & (1 << 26) == 1 << 26
    }

    // a3 field
    #[inline]
    pub fn get_a15(&self) -> PmpAField {
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
    pub fn get_l15(&self) -> bool {
        self.bits & (1 << 31) == 1 << 31
    }
}

read_csr_as!(Pmpcfg3, 0x3A3, __read_pmpcfg3);
set!(0x3A3, __set_pmpcfg3);
clear!(0x3A3, __clear_pmpcfg3);

set_clear_csr!(
    /// r0
    , set_r12, clear_r12, 1 << 0);
set_clear_csr!(
    /// w0
    , set_w12, clear_w12, 1 << 1);
set_clear_csr!(
    /// x0
    , set_x12, clear_x12, 1 << 2);
set_clear_csr!(
    /// l0
    , set_l12, clear_l12, 1 << 7);
set_clear_csr!(
    /// r1
    , set_r13, clear_r13, 1 << 8);
set_clear_csr!(
    /// w1
    , set_w13, clear_w13, 1 << 9);
set_clear_csr!(
    /// x1
    , set_x13, clear_x13, 1 << 10);
set_clear_csr!(
    /// l1
    , set_l13, clear_l13, 1 << 15);
set_clear_csr!(
    /// r2
    , set_r14, clear_r14, 1 << 16);
set_clear_csr!(
    /// w2
    , set_w14, clear_w14, 1 << 17);
set_clear_csr!(
    /// x2
    , set_x14, clear_x14, 1 << 18);
set_clear_csr!(
    /// l2
    , set_l14, clear_l14, 1 << 23);
set_clear_csr!(
    /// r3
    , set_r15, clear_r15, 1 << 24);
set_clear_csr!(
    /// w3
    , set_w15, clear_w15, 1 << 25);
set_clear_csr!(
    /// x3
    , set_x15, clear_x15, 1 << 26);
set_clear_csr!(
    /// l3
    , set_l15, clear_l15, 1 << 31);

#[inline]
pub unsafe fn set_a12(a_field: PmpAField) {
    _set((a_field as usize) << 3);
}
#[inline]
pub unsafe fn set_a13(a_field: PmpAField) {
    _set((a_field as usize) << 11);
}
#[inline]
pub unsafe fn set_a14(a_field: PmpAField) {
    _set((a_field as usize) << 19);
}
#[inline]
pub unsafe fn set_a15(a_field: PmpAField) {
    _set((a_field as usize) << 27);
}
