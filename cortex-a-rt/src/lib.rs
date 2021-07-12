#![feature(global_asm)]
#![feature(const_panic)]
#![feature(linkage)]
#![feature(asm)]
#![no_std]

use cortex_a::regs::RegisterReadOnly;
use register::Field;

pub mod exception;
pub mod memory;
pub mod mmu;

#[cfg(feature = "entry")]
pub mod entry {
    use cortex_a::{asm, regs::*};

    use crate::{exception, memory, mmu};

    // Initial boot handled by assembly
    global_asm!(include_str!("boot.s"));

    #[no_mangle]
    #[link_section = ".text._start_arguments"]
    pub static BOOT_CORE_ID: u64 = 0;

    /// Prepares the transition from EL2 to EL1.
    ///
    /// # Safety
    ///
    /// - The `bss` section is not initialized yet. The code must not use or reference it in any way.
    /// - The HW state of EL1 must be prepared in a sound way.
    #[inline(always)]
    unsafe fn prepare_el2_to_el1_transition(phys_boot_core_stack_end_exclusive_addr: u64) {
        // Enable timer counter registers for EL1.
        CNTHCTL_EL2.write(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);

        // No offset for reading the counters.
        CNTVOFF_EL2.set(0);

        // Set EL1 execution state to AArch64.
        HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);

        // Set up a simulated exception return.
        //
        // First, fake a saved program status where all interrupts were masked and SP_EL1 was used as a
        // stack pointer.
        SPSR_EL2.write(
            SPSR_EL2::D::Masked
                + SPSR_EL2::A::Masked
                + SPSR_EL2::I::Masked
                + SPSR_EL2::F::Masked
                + SPSR_EL2::M::EL1h,
        );

        // Second, let the link register point to _start_main().
        ELR_EL2.set(_start_main as *const () as u64);

        // Set up SP_EL1 (stack pointer), which will be used by EL1 once we "return" to it. Since there
        // are no plans to ever return to EL2, just re-use the same stack.
        SP_EL1.set(phys_boot_core_stack_end_exclusive_addr);
    }

    #[no_mangle]
    pub unsafe extern "C" fn _start_rust(phys_boot_core_stack_end_exclusive_addr: u64) -> ! {
        prepare_el2_to_el1_transition(phys_boot_core_stack_end_exclusive_addr);

        // Use `eret` to "return" to EL1. This results in execution of _start_main() in EL1.
        asm::eret()
    }

    #[no_mangle]
    pub unsafe extern "C" fn _start_main() -> ! {
        extern "Rust" {
            fn main() -> !;
        }

        memory::zero_bss();
        exception::handling_init();
        mmu::mmu()
            .enable_mmu_and_caching(&mmu::layout::default::default_layout())
            .expect("Failed to initialize MMU");

        main();
    }

    // Entry point is _start function in boot.s
    #[doc(hidden)]
    #[no_mangle]
    pub static __RPI_LOAD_ADDR: unsafe extern "C" fn() -> ! = _start;

    extern "C" {
        fn _start() -> !;
    }
}

/// Rertuns number of the core which is currently executing this function
pub fn core_id() -> u8 {
    cortex_a::regs::MPIDR_EL1.read(Field::<u64, ()>::new(0b11, 0x0)) as u8
}
