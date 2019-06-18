//! pmpaddr3 register
//!
//!
pub struct PmpAddr3 {
    bits: usize,
}

impl PmpAddr3 {
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(PmpAddr3, 0x3B3, __read_pmpaddr3);
write_csr!(0x3B3, __write_pmpaddr3);

#[inline]
pub unsafe fn write(addr: usize) {
    _write(addr);
}
