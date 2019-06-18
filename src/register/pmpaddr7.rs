//! pmpaddr7 register
//!
//!
pub struct PmpAddr7 {
    bits: usize,
}

impl PmpAddr7 {
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(PmpAddr7, 0x3B7, __read_pmpaddr7);
write_csr!(0x3B7, __write_pmpaddr7);

#[inline]
pub unsafe fn write(addr: usize) {
    _write(addr);
}
