#![no_std]
#![no_main]
#![feature(panic_info_message)]

/// panic handler
mod lang_items;
/// rustsbi service
mod sbi;
/// console
#[macro_use]
mod console;
/// batch
mod batch;
/// sync
mod sync;
/// trap
mod trap;
/// syscall
mod syscall;


use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello, World!");
    panic!("I paniced!")
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}