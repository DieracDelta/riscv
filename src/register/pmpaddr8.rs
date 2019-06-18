//! pmpaddr8 register
//!
//!
pub struct PmpAddr8 {
    bits: usize,
}

impl PmpAddr8 {
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(PmpAddr8, 0x3B8, __read_pmpaddr8);
write_csr!(0x3B8, __write_pmpaddr8);

#[inline]
pub unsafe fn write(addr: usize) {
    _write(addr);
}
