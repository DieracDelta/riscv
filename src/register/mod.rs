//! RISC-V CSR's
//!
//! The following registers are not available on 64-bit implementations.
//!
//! - cycleh
//! - timeh
//! - instreth
//! - hpmcounter[3-31]h
//! - mcycleh
//! - minstreth
//! - mhpmcounter[3-31]h

#[macro_use]
mod macros;

pub mod mcause;
pub mod mcycle;
pub mod mcycleh;
pub mod mepc;
pub mod mie;
pub mod minstret;
pub mod minstreth;
pub mod mip;
pub mod misa;
pub mod mstatus;
pub mod mtval;
pub mod mtvec;
pub mod mvendorid;
pub mod pmpaddr0;
pub mod pmpaddr1;
pub mod pmpaddr10;
pub mod pmpaddr11;
pub mod pmpaddr12;
pub mod pmpaddr13;
pub mod pmpaddr14;
pub mod pmpaddr15;
pub mod pmpaddr2;
pub mod pmpaddr3;
pub mod pmpaddr4;
pub mod pmpaddr5;
pub mod pmpaddr6;
pub mod pmpaddr7;
pub mod pmpaddr8;
pub mod pmpaddr9;
pub mod pmpcfg0;
pub mod pmpcfg1;
pub mod pmpcfg2;
pub mod pmpcfg3;

pub mod satp;
pub mod scause;
pub mod sepc;
pub mod sie;
pub mod sip;
pub mod sscratch;
pub mod sstatus;
pub mod stval;
pub mod stvec;

pub mod time;
pub mod timeh;

pub enum PmpAField {
    OFF = 0,
    TOR = 1,
    NA4 = 2,
    NAPOT = 3,
}
