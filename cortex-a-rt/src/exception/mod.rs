use cortex_a::{barrier, regs::*};

pub mod exception;
pub mod masking;

/// Init exception handling by setting the exception vector base address register.
///
/// # Safety
///
/// - Changes the HW state of the executing core.
/// - The vector table and the symbol `__exception_vector_table_start` from the linker script must
///   adhere to the alignment and size constraints demanded by the ARMv8-A Architecture Reference
///   Manual.
pub unsafe fn handling_init() {
    VBAR_EL1.set(exception::exception_vector_start() as u64);

    // Force VBAR update to complete before next instruction.
    barrier::isb(barrier::SY);
}
