use riscv::register::{sstatus::Sstatus, scause::Scause};

#[repr(C)]
pub struct Context {
    pub x: [usize, 32],
    pub sstatus: Sstatus,
    pub sepc: usize
}