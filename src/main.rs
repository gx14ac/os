#![no_std]
#![no_main]
#![warn(missing_docs)]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[macro_use]
mod console;
mod interrupt;
mod memory;
mod panic;
mod sbi;

extern crate alloc;

global_asm!(include_str!("asm/entry.asm"));

/// Runtime Override
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println!("Hello!");
    interrupt::init();
    memory::init();

    use alloc::boxed::Box;
    use alloc::vec::Vec;

    let v = Box::new(5);
    assert_eq!(*v, 5);
    let mut vec = Vec::new();

    for i in 0..10000 {
        vec.push(i);
    }

    for i in 0..10000 {
        assert_eq!(vec[i], i);
    }

    println!("heap test passed");

    unsafe {
        // 割り込み処理発生
        llvm_asm!("ebreak"::::"volatile");
    };

    loop {}
}
