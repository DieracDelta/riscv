//Approved for public release. Distribution is unlimited.

//This material is based upon work supported by the Under Secretary of Defense for Research and Engineering under Air Force Contract No. FA8702-15-D-0001. Any opinions, findings, conclusions or recommendations expressed in this material are those of the author(s) and do not necessarily reflect the views of the Under Secretary of Defense for Research and Engineering.

//© 2019 Massachusetts Institute of Technology.

//The software/firmware is provided to you on an As-Is basis

//Delivered to the U.S. Government with Unlimited Rights, as defined in DFARS Part 252.227-7013 or 7014 (Feb 2014). Notwithstanding any copyright notice, U.S. Government rights in this work are defined by DFARS 252.227-7013 or DFARS 252.227-7014 as detailed above. Use of this work other than as specifically authorized by the U.S. Government may violate any copyrights that exist in this work.

use register::PmpAField;
// only meant for 32 bit
pub struct Pmpcfg0 {
    bits: usize,
}

impl Pmpcfg0 {
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    // r0 field
    #[inline]
    pub fn get_r0(&self) -> bool {
        self.bits & (1 << 0) == 1 << 0
    }

    // w0 field
    #[inline]
    pub fn get_w0(&self) -> bool {
        self.bits & (1 << 1) == 1 << 1
    }

    // x0 field
    #[inline]
    pub fn get_x0(&self) -> bool {
        self.bits & (1 << 2) == 1 << 2
    }

    // a0 field
    #[inline]
    pub fn get_a0(&self) -> PmpAField {
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
    pub fn get_l0(&self) -> bool {
        self.bits & (1 << 7) == 1 << 7
    }

    // r1 field
    #[inline]
    pub fn get_r1(&self) -> bool {
        self.bits & (1 << 8) == 1 << 8
    }

    // w1 field
    #[inline]
    pub fn get_w1(&self) -> bool {
        self.bits & (1 << 9) == 1 << 9
    }

    // x1 field
    #[inline]
    pub fn get_x1(&self) -> bool {
        self.bits & (1 << 10) == 1 << 10
    }

    // a1 field
    #[inline]
    pub fn get_a1(&self) -> PmpAField {
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
    pub fn get_l1(&self) -> bool {
        self.bits & (1 << 15) == 1 << 15
    }

    // r2 field
    #[inline]
    pub fn get_r2(&self) -> bool {
        self.bits & (1 << 16) == 1 << 16
    }

    // w2 field
    #[inline]
    pub fn get_w2(&self) -> bool {
        self.bits & (1 << 17) == 1 << 17
    }

    // x2 field
    #[inline]
    pub fn get_x2(&self) -> bool {
        self.bits & (1 << 18) == 1 << 18
    }

    // a2 field
    #[inline]
    pub fn get_a2(&self) -> PmpAField {
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
    pub fn get_l2(&self) -> bool {
        self.bits & (1 << 23) == 1 << 23
    }

    // r3 field
    #[inline]
    pub fn get_r3(&self) -> bool {
        self.bits & (1 << 24) == 1 << 24
    }

    // w3 field
    #[inline]
    pub fn get_w3(&self) -> bool {
        self.bits & (1 << 25) == 1 << 25
    }

    // x3 field
    #[inline]
    pub fn get_x3(&self) -> bool {
        self.bits & (1 << 26) == 1 << 26
    }

    // a3 field
    #[inline]
    pub fn get_a3(&self) -> PmpAField {
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
    pub fn get_l3(&self) -> bool {
        self.bits & (1 << 31) == 1 << 31
    }
}

read_csr_as!(Pmpcfg0, 0x3A0, __read_pmpcfg0);
set!(0x3A0, __set_pmpcfg0);
clear!(0x3A0, __clear_pmpcfg0);

set_clear_csr!(
    /// r0
    , set_r0, clear_r0, 1 << 0);
set_clear_csr!(
    /// w0
    , set_w0, clear_w0, 1 << 1);
set_clear_csr!(
    /// x0
    , set_x0, clear_x0, 1 << 2);
set_clear_csr!(
    /// l0
    , set_l0, clear_l0, 1 << 7);
set_clear_csr!(
    /// r1
    , set_r1, clear_r1, 1 << 8);
set_clear_csr!(
    /// w1
    , set_w1, clear_w1, 1 << 9);
set_clear_csr!(
    /// x1
    , set_x1, clear_x1, 1 << 10);
set_clear_csr!(
    /// l1
    , set_l1, clear_l1, 1 << 15);
set_clear_csr!(
    /// r2
    , set_r2, clear_r2, 1 << 16);
set_clear_csr!(
    /// w2
    , set_w2, clear_w2, 1 << 17);
set_clear_csr!(
    /// x2
    , set_x2, clear_x2, 1 << 18);
set_clear_csr!(
    /// l2
    , set_l2, clear_l2, 1 << 23);
set_clear_csr!(
    /// r3
    , set_r3, clear_r3, 1 << 24);
set_clear_csr!(
    /// w3
    , set_w3, clear_w3, 1 << 25);
set_clear_csr!(
    /// x3
    , set_x3, clear_x3, 1 << 26);
set_clear_csr!(
    /// l3
    , set_l3, clear_l3, 1 << 31);

#[inline]
pub unsafe fn set_a0(a_field: PmpAField) {
    _set((a_field as usize) << 3);
}
#[inline]
pub unsafe fn set_a1(a_field: PmpAField) {
    _set((a_field as usize) << 11);
}
#[inline]
pub unsafe fn set_a2(a_field: PmpAField) {
    _set((a_field as usize) << 19);
}
#[inline]
pub unsafe fn set_a3(a_field: PmpAField) {
    _set((a_field as usize) << 27);
}
