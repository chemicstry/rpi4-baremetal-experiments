use cortex_a::regs::{RegisterReadWrite, DAIF};
use register::LocalRegisterCopy;

pub mod daif_bits {
    pub const DEBUG: u8 = 0b1000;
    pub const SERROR: u8 = 0b0100;
    pub const IRQ: u8 = 0b0010;
    pub const FIQ: u8 = 0b0001;
}

/// Unmask debug interrupts (Watchpoint, Breakpoint, and Software Step exceptions) on the executing core.
///
/// # Safety
///
/// - Changes the HW state of the executing core.
#[inline(always)]
pub unsafe fn local_debug_unmask() {
    // It is not needed to place an explicit instruction synchronization barrier after the `msr`.
    // Quoting the Architecture Reference Manual for ARMv8-A, section C5.1.3:
    //
    // "Writes to PSTATE.{PAN, D, A, I, F} occur in program order without the need for additional
    // synchronization."
    #[rustfmt::skip]
    asm!(
        "msr DAIFClr, {arg}",
        arg = const daif_bits::DEBUG,
        options(nomem, nostack, preserves_flags)
    );
}

/// Mask debug interrupts (Watchpoint, Breakpoint, and Software Step exceptions) on the executing core.
///
/// # Safety
///
/// - Changes the HW state of the executing core.
#[inline(always)]
pub unsafe fn local_debug_mask() {
    #[rustfmt::skip]
    asm!(
        "msr DAIFSet, {arg}",
        arg = const daif_bits::DEBUG,
        options(nomem, nostack, preserves_flags)
    );
}

/// Unmask SError interrupts on the executing core.
///
/// # Safety
///
/// - Changes the HW state of the executing core.
#[inline(always)]
pub unsafe fn local_serror_unmask() {
    #[rustfmt::skip]
    asm!(
        "msr DAIFClr, {arg}",
        arg = const daif_bits::SERROR,
        options(nomem, nostack, preserves_flags)
    );
}

/// Mask SError interrupts on the executing core.
///
/// # Safety
///
/// - Changes the HW state of the executing core.
#[inline(always)]
pub unsafe fn local_serror_mask() {
    #[rustfmt::skip]
    asm!(
        "msr DAIFSet, {arg}",
        arg = const daif_bits::SERROR,
        options(nomem, nostack, preserves_flags)
    );
}

/// Unmask IRQs on the executing core.
///
/// # Safety
///
/// - Changes the HW state of the executing core.
#[inline(always)]
pub unsafe fn local_irq_unmask() {
    #[rustfmt::skip]
    asm!(
        "msr DAIFClr, {arg}",
        arg = const daif_bits::IRQ,
        options(nomem, nostack, preserves_flags)
    );
}

/// Mask IRQs on the executing core.
///
/// # Safety
///
/// - Changes the HW state of the executing core.
#[inline(always)]
pub unsafe fn local_irq_mask() {
    #[rustfmt::skip]
    asm!(
        "msr DAIFSet, {arg}",
        arg = const daif_bits::IRQ,
        options(nomem, nostack, preserves_flags)
    );
}

/// Unmask FIQs on the executing core.
///
/// # Safety
///
/// - Changes the HW state of the executing core.
#[inline(always)]
pub unsafe fn local_fiq_unmask() {
    #[rustfmt::skip]
    asm!(
        "msr DAIFClr, {arg}",
        arg = const daif_bits::FIQ,
        options(nomem, nostack, preserves_flags)
    );
}

/// Mask FIQs on the executing core.
///
/// # Safety
///
/// - Changes the HW state of the executing core.
#[inline(always)]
pub unsafe fn local_fiq_mask() {
    #[rustfmt::skip]
    asm!(
        "msr DAIFSet, {arg}",
        arg = const daif_bits::FIQ,
        options(nomem, nostack, preserves_flags)
    );
}

/// Contains the interrupt mask state
pub struct DaifState(LocalRegisterCopy<u64, DAIF::Register>);

impl core::fmt::Debug for DaifState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "Exception handling state:")?;
        writeln!(f, "    Debug  (D): {}", self.0.is_set(DAIF::D))?;
        writeln!(f, "    SError (A): {}", self.0.is_set(DAIF::A))?;
        writeln!(f, "    IRQ    (I): {}", self.0.is_set(DAIF::I))?;
        writeln!(f, "    FIQ    (F): {}", self.0.is_set(DAIF::F))
    }
}

/// Returns the current mask state of the executing core
///
/// # Safety
///
/// - Changes the HW state of the executing core.
#[inline(always)]
pub unsafe fn local_mask_save() -> DaifState {
    DaifState(DAIF.extract())
}

/// Restores saved interrupt mask state on the executing core
///
/// # Safety
///
/// - Changes the HW state of the executing core.
#[inline(always)]
pub unsafe fn local_mask_restore(state: DaifState) {
    DAIF.set(state.0.get())
}
