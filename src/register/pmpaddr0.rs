//! pmpaddr0 register
//!
//!
pub struct PmpAddr0 {
    bits: usize,
}

impl PmpAddr0 {
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(PmpAddr0, 0x3B0, __read_pmpaddr0);
write_csr!(0x3B0, __write_pmpaddr0);

#[inline]
pub unsafe fn write(addr: usize) {
    _write(addr);
}
