//! pmpaddr4 register
//!
//!
pub struct PmpAddr4 {
    bits: usize,
}

impl PmpAddr4 {
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(PmpAddr4, 0x3B4, __read_pmpaddr4);
write_csr!(0x3B4, __write_pmpaddr4);

#[inline]
pub unsafe fn write(addr: usize) {
    _write(addr);
}
