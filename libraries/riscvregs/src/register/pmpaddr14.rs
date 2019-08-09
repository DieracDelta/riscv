//Approved for public release. Distribution is unlimited.

//This material is based upon work supported by the Under Secretary of Defense for Research and Engineering under Air Force Contract No. FA8702-15-D-0001. Any opinions, findings, conclusions or recommendations expressed in this material are those of the author(s) and do not necessarily reflect the views of the Under Secretary of Defense for Research and Engineering.

//© 2019 Massachusetts Institute of Technology.

//The software/firmware is provided to you on an As-Is basis

//Delivered to the U.S. Government with Unlimited Rights, as defined in DFARS Part 252.227-7013 or 7014 (Feb 2014). Notwithstanding any copyright notice, U.S. Government rights in this work are defined by DFARS 252.227-7013 or DFARS 252.227-7014 as detailed above. Use of this work other than as specifically authorized by the U.S. Government may violate any copyrights that exist in this work.

//! pmpaddr14 register
//!
//!
pub struct PmpAddr14 {
    bits: usize,
}

impl PmpAddr14 {
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(PmpAddr14, 0x3BE, __read_pmpaddr14);
write_csr!(0x3BE, __write_pmpaddr14);

#[inline]
pub unsafe fn write(addr: usize) {
    _write(addr);
}
