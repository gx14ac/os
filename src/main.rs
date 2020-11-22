#![no_std]
#![no_main]
#![warn(missing_docs)]
#![feature(llvm_asm)]
#![feature(global_asm)]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod interrupt;
mod panic;
mod sbi;

global_asm!(include_str!("asm/entry.asm"));

/// Runtime Override
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println!("Hello!");
    interrupt::init();

    unsafe {
        // 割り込み処理発生
        llvm_asm!("ebreak"::::"volatile");
    };

    loop {}
}
