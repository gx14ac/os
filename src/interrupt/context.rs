use riscv::register::{scause::Scause, sstatus::Sstatus};

#[repr(C)]
pub struct Context {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}
