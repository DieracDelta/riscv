//Approved for public release. Distribution is unlimited.

//This material is based upon work supported by the Under Secretary of Defense for Research and Engineering under Air Force Contract No. FA8702-15-D-0001. Any opinions, findings, conclusions or recommendations expressed in this material are those of the author(s) and do not necessarily reflect the views of the Under Secretary of Defense for Research and Engineering.

//© 2019 Massachusetts Institute of Technology.

//The software/firmware is provided to you on an As-Is basis

//Delivered to the U.S. Government with Unlimited Rights, as defined in DFARS Part 252.227-7013 or 7014 (Feb 2014). Notwithstanding any copyright notice, U.S. Government rights in this work are defined by DFARS 252.227-7013 or DFARS 252.227-7014 as detailed above. Use of this work other than as specifically authorized by the U.S. Government may violate any copyrights that exist in this work.

use register::PmpAField;
// only meant for 32 bit
pub struct Pmpcfg1 {
    bits: usize,
}

impl Pmpcfg1 {
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    // r0 field
    #[inline]
    pub fn get_r4(&self) -> bool {
        self.bits & (1 << 0) == 1 << 0
    }

    // w0 field
    #[inline]
    pub fn get_w4(&self) -> bool {
        self.bits & (1 << 1) == 1 << 1
    }

    // x0 field
    #[inline]
    pub fn get_x4(&self) -> bool {
        self.bits & (1 << 2) == 1 << 2
    }

    // a0 field
    #[inline]
    pub fn get_a4(&self) -> PmpAField {
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
    pub fn get_l4(&self) -> bool {
        self.bits & (1 << 7) == 1 << 7
    }

    // r1 field
    #[inline]
    pub fn get_r5(&self) -> bool {
        self.bits & (1 << 8) == 1 << 8
    }

    // w1 field
    #[inline]
    pub fn get_w5(&self) -> bool {
        self.bits & (1 << 9) == 1 << 9
    }

    // x1 field
    #[inline]
    pub fn get_x5(&self) -> bool {
        self.bits & (1 << 10) == 1 << 10
    }

    // a1 field
    #[inline]
    pub fn get_a5(&self) -> PmpAField {
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
    pub fn get_l5(&self) -> bool {
        self.bits & (1 << 15) == 1 << 15
    }

    // r2 field
    #[inline]
    pub fn get_r6(&self) -> bool {
        self.bits & (1 << 16) == 1 << 16
    }

    // w2 field
    #[inline]
    pub fn get_w6(&self) -> bool {
        self.bits & (1 << 17) == 1 << 17
    }

    // x2 field
    #[inline]
    pub fn get_x6(&self) -> bool {
        self.bits & (1 << 18) == 1 << 18
    }

    // a2 field
    #[inline]
    pub fn get_a6(&self) -> PmpAField {
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
    pub fn get_l6(&self) -> bool {
        self.bits & (1 << 23) == 1 << 23
    }

    // r3 field
    #[inline]
    pub fn get_r7(&self) -> bool {
        self.bits & (1 << 24) == 1 << 24
    }

    // w3 field
    #[inline]
    pub fn get_w7(&self) -> bool {
        self.bits & (1 << 25) == 1 << 25
    }

    // x3 field
    #[inline]
    pub fn get_x7(&self) -> bool {
        self.bits & (1 << 26) == 1 << 26
    }

    // a3 field
    #[inline]
    pub fn get_a7(&self) -> PmpAField {
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
    pub fn get_l7(&self) -> bool {
        self.bits & (1 << 31) == 1 << 31
    }
}

read_csr_as!(Pmpcfg1, 0x3A1, __read_pmpcfg1);
set!(0x3A1, __set_pmpcfg1);
clear!(0x3A1, __clear_pmpcfg1);

set_clear_csr!(
    /// r0
    , set_r4, clear_r4, 1 << 0);
set_clear_csr!(
    /// w0
    , set_w4, clear_w4, 1 << 1);
set_clear_csr!(
    /// x0
    , set_x4, clear_x4, 1 << 2);
set_clear_csr!(
    /// l0
    , set_l4, clear_l4, 1 << 7);
set_clear_csr!(
    /// r1
    , set_r5, clear_r5, 1 << 8);
set_clear_csr!(
    /// w1
    , set_w5, clear_w5, 1 << 9);
set_clear_csr!(
    /// x1
    , set_x5, clear_x5, 1 << 10);
set_clear_csr!(
    /// l1
    , set_l5, clear_l5, 1 << 15);
set_clear_csr!(
    /// r2
    , set_r6, clear_r6, 1 << 16);
set_clear_csr!(
    /// w2
    , set_w6, clear_w6, 1 << 17);
set_clear_csr!(
    /// x2
    , set_x6, clear_x6, 1 << 18);
set_clear_csr!(
    /// l2
    , set_l6, clear_l6, 1 << 23);
set_clear_csr!(
    /// r3
    , set_r7, clear_r7, 1 << 24);
set_clear_csr!(
    /// w3
    , set_w7, clear_w7, 1 << 25);
set_clear_csr!(
    /// x3
    , set_x7, clear_x7, 1 << 26);
set_clear_csr!(
    /// l3
    , set_l7, clear_l7, 1 << 31);

#[inline]
pub unsafe fn set_a4(a_field: PmpAField) {
    _set((a_field as usize) << 3);
}
#[inline]
pub unsafe fn set_a5(a_field: PmpAField) {
    _set((a_field as usize) << 11);
}
#[inline]
pub unsafe fn set_a6(a_field: PmpAField) {
    _set((a_field as usize) << 19);
}
#[inline]
pub unsafe fn set_a7(a_field: PmpAField) {
    _set((a_field as usize) << 27);
}
