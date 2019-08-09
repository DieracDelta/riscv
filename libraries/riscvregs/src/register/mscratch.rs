/// mscratch register
#[derive(Clone, Copy, Debug)]
pub struct Mscratch {
    bits: usize,
}
read_csr_as!(Mscratch, 0x340, __read_mscratch);
set!(0x340, __set_mscratch);
clear!(0x340, __clear_mscratch);
