//! mepc register
//!
//!
pub struct Mepc {
    bits: usize,
}

impl Mepc {
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(Mepc, 0x341, __read_mepc);
write_csr!(0x341, __write_mepc);

#[inline]
pub unsafe fn write(addr: usize) {
    _write(addr);
}
