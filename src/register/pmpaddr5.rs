//! pmpaddr5 register
//!
//!
pub struct PmpAddr5 {
    bits: usize,
}

impl PmpAddr5 {
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(PmpAddr5, 0x3B5, __read_pmpaddr5);
write_csr!(0x3B5, __write_pmpaddr5);

#[inline]
pub unsafe fn write(addr: usize) {
    _write(addr);
}
