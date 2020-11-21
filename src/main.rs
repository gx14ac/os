#![no_std]
#![no_main]
use core::panic::PanicInfo;

/*
    標準のpanicが使えないため、
    panic_handlerを明示的に使用 (Cargo.tomlのprofileセクションに定義済み)
*/
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/* override to crt0 */
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
