//Approved for public release. Distribution is unlimited.

//This material is based upon work supported by the Under Secretary of Defense for Research and Engineering under Air Force Contract No. FA8702-15-D-0001. Any opinions, findings, conclusions or recommendations expressed in this material are those of the author(s) and do not necessarily reflect the views of the Under Secretary of Defense for Research and Engineering.

//© 2019 Massachusetts Institute of Technology.

//The software/firmware is provided to you on an As-Is basis

//Delivered to the U.S. Government with Unlimited Rights, as defined in DFARS Part 252.227-7013 or 7014 (Feb 2014). Notwithstanding any copyright notice, U.S. Government rights in this work are defined by DFARS 252.227-7013 or DFARS 252.227-7014 as detailed above. Use of this work other than as specifically authorized by the U.S. Government may violate any copyrights that exist in this work.

//! pmpaddr12 register
//!
//!
pub struct PmpAddr12 {
    bits: usize,
}

impl PmpAddr12 {
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(PmpAddr12, 0x3BC, __read_pmpaddr12);
write_csr!(0x3BC, __write_pmpaddr12);

#[inline]
pub unsafe fn write(addr: usize) {
    _write(addr);
}
