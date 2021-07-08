// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2021 Andre Richter <andre.o.richter@gmail.com>

use core::{cell::UnsafeCell, fmt};
use cortex_a::{barrier, regs::*};
use register::InMemoryRegister;

// Assembly counterpart to this file.
global_asm!(include_str!("exception.s"));

//--------------------------------------------------------------------------------------------------
// Private Definitions
//--------------------------------------------------------------------------------------------------

/// Wrapper struct for memory copy of SPSR_EL1.
#[repr(transparent)]
struct SpsrEL1(InMemoryRegister<u64, SPSR_EL1::Register>);

/// The exception context as it is stored on the stack on exception entry.
#[repr(C)]
struct ExceptionContext {
    /// General Purpose Registers.
    gpr: [u64; 30],

    /// The link register, aka x30.
    lr: u64,

    /// Exception link register. The program counter at the time the exception happened.
    elr_el1: u64,

    /// Saved program status.
    spsr_el1: SpsrEL1,
}

/// Wrapper struct for pretty printing ESR_EL1.
struct EsrEL1;

//--------------------------------------------------------------------------------------------------
// Private Code
//--------------------------------------------------------------------------------------------------

/// Prints verbose information about the exception and then panics.
fn default_exception_handler(e: &ExceptionContext) {
    panic!(
        "\n\nCPU Exception!\n\
         FAR_EL1: {:#018x}\n\
         {}\n\
         {}",
        FAR_EL1.get(),
        EsrEL1 {},
        e
    );
}

//------------------------------------------------------------------------------
// Current, EL0
//------------------------------------------------------------------------------

#[no_mangle]
#[linkage = "weak"]
unsafe extern "C" fn current_el0_synchronous(e: &mut ExceptionContext) {
    default_exception_handler(e);
}

#[no_mangle]
#[linkage = "weak"]
unsafe extern "C" fn current_el0_irq(e: &mut ExceptionContext) {
    default_exception_handler(e);
}

#[no_mangle]
#[linkage = "weak"]
unsafe extern "C" fn current_el0_serror(e: &mut ExceptionContext) {
    default_exception_handler(e);
}

//------------------------------------------------------------------------------
// Current, ELx
//------------------------------------------------------------------------------

#[no_mangle]
#[linkage = "weak"]
unsafe extern "C" fn current_elx_synchronous(e: &mut ExceptionContext) {
    default_exception_handler(e);
}

#[no_mangle]
#[linkage = "weak"]
unsafe extern "C" fn current_elx_irq(e: &mut ExceptionContext) {
    default_exception_handler(e);
}

#[no_mangle]
#[linkage = "weak"]
unsafe extern "C" fn current_elx_serror(e: &mut ExceptionContext) {
    default_exception_handler(e);
}

//------------------------------------------------------------------------------
// Lower, AArch64
//------------------------------------------------------------------------------

#[no_mangle]
#[linkage = "weak"]
unsafe extern "C" fn lower_aarch64_synchronous(e: &mut ExceptionContext) {
    default_exception_handler(e);
}

#[no_mangle]
#[linkage = "weak"]
unsafe extern "C" fn lower_aarch64_irq(e: &mut ExceptionContext) {
    default_exception_handler(e);
}

#[no_mangle]
#[linkage = "weak"]
unsafe extern "C" fn lower_aarch64_serror(e: &mut ExceptionContext) {
    default_exception_handler(e);
}

//------------------------------------------------------------------------------
// Lower, AArch32
//------------------------------------------------------------------------------

#[no_mangle]
#[linkage = "weak"]
unsafe extern "C" fn lower_aarch32_synchronous(e: &mut ExceptionContext) {
    default_exception_handler(e);
}

#[no_mangle]
#[linkage = "weak"]
unsafe extern "C" fn lower_aarch32_irq(e: &mut ExceptionContext) {
    default_exception_handler(e);
}

#[no_mangle]
#[linkage = "weak"]
unsafe extern "C" fn lower_aarch32_serror(e: &mut ExceptionContext) {
    default_exception_handler(e);
}

//------------------------------------------------------------------------------
// Pretty printing
//------------------------------------------------------------------------------

/// Human readable ESR_EL1.
#[rustfmt::skip]
impl fmt::Display for EsrEL1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let esr_el1 = ESR_EL1.extract();

        // Raw print of whole register.
        writeln!(f, "ESR_EL1: {:#010x}", esr_el1.get())?;

        // Raw print of exception class.
        write!(f, "      Exception Class         (EC) : {:#x}", esr_el1.read(ESR_EL1::EC))?;

        // Exception class, translation.
        let ec_translation = match esr_el1.read_as_enum(ESR_EL1::EC) {
            Some(ESR_EL1::EC::Value::DataAbortCurrentEL) => "Data Abort, current EL",
            _ => "N/A",
        };
        writeln!(f, " - {}", ec_translation)?;

        // Raw print of instruction specific syndrome.
        write!(f, "      Instr Specific Syndrome (ISS): {:#x}", esr_el1.read(ESR_EL1::ISS))
    }
}

/// Human readable SPSR_EL1.
#[rustfmt::skip]
impl fmt::Display for SpsrEL1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Raw value.
        writeln!(f, "SPSR_EL1: {:#010x}", self.0.get())?;

        let to_flag_str = |x| -> _ {
            if x { "Set" } else { "Not set" }
         };

        writeln!(f, "      Flags:")?;
        writeln!(f, "            Negative (N): {}", to_flag_str(self.0.is_set(SPSR_EL1::N)))?;
        writeln!(f, "            Zero     (Z): {}", to_flag_str(self.0.is_set(SPSR_EL1::Z)))?;
        writeln!(f, "            Carry    (C): {}", to_flag_str(self.0.is_set(SPSR_EL1::C)))?;
        writeln!(f, "            Overflow (V): {}", to_flag_str(self.0.is_set(SPSR_EL1::V)))?;

        let to_mask_str = |x| -> _ {
            if x { "Masked" } else { "Unmasked" }
        };

        writeln!(f, "      Exception handling state:")?;
        writeln!(f, "            Debug  (D): {}", to_mask_str(self.0.is_set(SPSR_EL1::D)))?;
        writeln!(f, "            SError (A): {}", to_mask_str(self.0.is_set(SPSR_EL1::A)))?;
        writeln!(f, "            IRQ    (I): {}", to_mask_str(self.0.is_set(SPSR_EL1::I)))?;
        writeln!(f, "            FIQ    (F): {}", to_mask_str(self.0.is_set(SPSR_EL1::F)))?;

        write!(f, "      Illegal Execution State (IL): {}",
            to_flag_str(self.0.is_set(SPSR_EL1::IL))
        )
    }
}

/// Human readable print of the exception context.
impl fmt::Display for ExceptionContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "ELR_EL1: {:#018x}", self.elr_el1)?;
        writeln!(f, "{}", self.spsr_el1)?;
        writeln!(f)?;
        writeln!(f, "General purpose register:")?;

        #[rustfmt::skip]
        let alternating = |x| -> _ {
            if x % 2 == 0 { "   " } else { "\n" }
        };

        // Print two registers per line.
        for (i, reg) in self.gpr.iter().enumerate() {
            write!(f, "      x{: <2}: {: >#018x}{}", i, reg, alternating(i))?;
        }
        write!(f, "      lr : {:#018x}", self.lr)
    }
}

/// Kernel privilege levels.
#[derive(PartialEq)]
pub enum PrivilegeLevel {
    User,
    Kernel,
    Hypervisor,
    Unknown,
}

/// The processing element's current privilege level.
pub fn current_privilege_level() -> (PrivilegeLevel, &'static str) {
    let el = CurrentEL.read_as_enum(CurrentEL::EL);
    match el {
        Some(CurrentEL::EL::Value::EL2) => (PrivilegeLevel::Hypervisor, "EL2"),
        Some(CurrentEL::EL::Value::EL1) => (PrivilegeLevel::Kernel, "EL1"),
        Some(CurrentEL::EL::Value::EL0) => (PrivilegeLevel::User, "EL0"),
        _ => (PrivilegeLevel::Unknown, "Unknown"),
    }
}

/// Init exception handling by setting the exception vector base address register.
///
/// # Safety
///
/// - Changes the HW state of the executing core.
/// - The vector table and the symbol `__exception_vector_table_start` from the linker script must
///   adhere to the alignment and size constraints demanded by the ARMv8-A Architecture Reference
///   Manual.
pub unsafe fn handling_init() {
    // Provided by exception.S.
    extern "Rust" {
        static __exception_vector_start: UnsafeCell<()>;
    }

    VBAR_EL1.set(__exception_vector_start.get() as u64);

    // Force VBAR update to complete before next instruction.
    barrier::isb(barrier::SY);
}

pub mod masking {
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
}
