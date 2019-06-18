//! pmpaddr11 register
//!
//!
pub struct PmpAddr11 {
    bits: usize,
}

impl PmpAddr11 {
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(PmpAddr11, 0x3BB, __read_pmpaddr11);
write_csr!(0x3BB, __write_pmpaddr11);

#[inline]
pub unsafe fn write(addr: usize) {
    _write(addr);
}
