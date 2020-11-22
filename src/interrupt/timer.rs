use crate::sbi::set_timer;
use riscv::register::{sie, sstatus, time};

static TIMER_CLOCK_INTERVAL: usize = 100000;
pub static mut TICKS: usize = 0;

// クロック割り込みの有効化
pub fn init() {
    unsafe {
        sie::set_stimer();
        sstatus::set_sie();
    }

    set_next_timeout();
}

// 現在の時刻と割り込み間隔を取得し、SBIシステムコールで次の割り込み時間をスケジューリングする
fn set_next_timeout() {
    set_timer(time::read() + TIMER_CLOCK_INTERVAL);
}

// クロックが途切れるたびに設定する
pub fn tick() {
    set_next_timeout();
    unsafe {
        TICKS += 1;
        if TICKS % 100 == 0 {
            println!("100 ticks~");
        }
    }
}
