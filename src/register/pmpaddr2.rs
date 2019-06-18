//! pmpaddr2 register
//!
//!
pub struct PmpAddr2 {
    bits: usize,
}

impl PmpAddr2 {
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(PmpAddr2, 0x3B2, __read_pmpaddr2);
write_csr!(0x3B2, __write_pmpaddr2);

#[inline]
pub unsafe fn write(addr: usize) {
    _write(addr);
}
