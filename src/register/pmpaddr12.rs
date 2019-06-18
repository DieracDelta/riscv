//! pmpaddr13 register
//!
//!
pub struct PmpAddr13 {
    bits: usize,
}

impl PmpAddr13 {
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(PmpAddr13, 0x3BC, __read_pmpaddr13);
write_csr!(0x3BC, __write_pmpaddr13);

#[inline]
pub unsafe fn write(addr: usize) {
    _write(addr);
}
