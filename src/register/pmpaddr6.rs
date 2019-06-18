//! pmpaddr6 register
//!
//!
pub struct PmpAddr6 {
    bits: usize,
}

impl PmpAddr6 {
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(PmpAddr6, 0x3B6, __read_pmpaddr6);
write_csr!(0x3B6, __write_pmpaddr6);

#[inline]
pub unsafe fn write(addr: usize) {
    _write(addr);
}
