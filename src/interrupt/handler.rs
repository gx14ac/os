use super::context::Context;
use super::timer;
use riscv::register::{
    scause::{Exception, Interrupt, Scause, Trap},
    stvec,
};

global_asm!(include_str!("../asm/interrupt.asm"));

/*
    `__interrupt` を `stvec` に書き込み、割り込み許可を行う
*/
pub fn init() {
    unsafe {
        extern "C" {
            // `interrupt.asm`の割り込み関数
            fn __interrupt();
        }

        // ダイレクトモードでの割り込み処理を`__interrupt`にする
        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);
    }
}

#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
    // 割り込み処理に応じて処理を分ける
    match scause.cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        _ => unimplemented!(
            "{:?}: {:x}, stval: 0x{:x}",
            scause.cause(),
            context.sepc,
            stval
        ),
    }
}

// `ebreak` コマンドをスキップするために 2 バイトを追加する
fn breakpoint(context: &mut Context) {
    println!("Breakpoint at 0x{:x}", context.sepc);
    context.sepc += 2;
}

fn supervisor_timer(_: &Context) {
    timer::tick();
}
