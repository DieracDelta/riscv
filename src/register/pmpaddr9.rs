//! pmpaddr9 register
//!
//!
pub struct PmpAddr9 {
    bits: usize,
}

impl PmpAddr9 {
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(PmpAddr9, 0x3B9, __read_pmpaddr9);
write_csr!(0x3B9, __write_pmpaddr9);

#[inline]
pub unsafe fn write(addr: usize) {
    _write(addr);
}
