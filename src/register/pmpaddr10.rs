//! pmpaddr10 register
//!
//!
pub struct PmpAddr10 {
    bits: usize,
}

impl PmpAddr10 {
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(PmpAddr10, 0x3BA, __read_pmpaddr10);
write_csr!(0x3BA, __write_pmpaddr10);

#[inline]
pub unsafe fn write(addr: usize) {
    _write(addr);
}
