#![allow(dead_code)]
pub mod config;
pub mod heap;

pub fn init() {
    heap::init();
    println!("mod memory initialized");
}
