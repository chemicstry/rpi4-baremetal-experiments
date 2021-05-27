//use crate::memory::zero_bss;

// Initial boot handled by assembly
global_asm!(include_str!("boot.s"));

#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static BOOT_CORE_ID: u64 = 0;

#[no_mangle]
pub unsafe fn _start_rust() -> ! {
    extern "Rust" {
        fn main() -> !;
    }

    //zero_bss();
    main();
}

