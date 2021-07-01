#![feature(global_asm)]
#![no_std]

mod memory;
use crate::memory::zero_bss;

// Initial boot handled by assembly
global_asm!(include_str!("boot.s"));

#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static BOOT_CORE_ID: u64 = 0;

#[no_mangle]
pub unsafe extern "C" fn _start_rust() -> ! {
    extern "Rust" {
        fn main() -> !;
    }

    zero_bss();
    main();
}

// Entry point is _start function in boot.s
#[doc(hidden)]
#[no_mangle]
pub static __RPI_LOAD_ADDR: unsafe extern "C" fn() -> ! = _start;

extern "C" {
    fn _start() -> !;
}
