//! pmpaddr1 register
//!
//!
pub struct PmpAddr1 {
    bits: usize,
}

impl PmpAddr1 {
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(PmpAddr1, 0x3B1, __read_pmpaddr1);
write_csr!(0x3B1, __write_pmpaddr1);

#[inline]
pub unsafe fn write(addr: usize) {
    _write(addr);
}
