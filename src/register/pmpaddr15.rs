//! pmpaddr15 register
//!
//!
pub struct PmpAddr15 {
    bits: usize,
}

impl PmpAddr15 {
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(PmpAddr15, 0x3BF, __read_pmpaddr15);
write_csr!(0x3BF, __write_pmpaddr15);

#[inline]
pub unsafe fn write(addr: usize) {
    _write(addr);
}
