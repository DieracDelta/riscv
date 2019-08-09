//! uepc register
//!
//!
pub struct Uepc {
    bits: usize,
}

impl Uepc {
    pub fn bits(&self) -> usize {
        self.bits
    }
}

read_csr_as!(Uepc, 0x341, __read_uepc);
write_csr!(0x341, __write_uepc);

#[inline]
pub unsafe fn write(addr: usize) {
    _write(addr);
}
