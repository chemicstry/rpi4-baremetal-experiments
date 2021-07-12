use core::{cell::UnsafeCell, ops::RangeInclusive};

use crate::mmu::{
    AccessPermissions, AddressSpace, AttributeFields, MemAttributes, Translation,
    TranslationDescriptor,
};

use super::{simple::SimpleMemoryLayout, VirtualMemoryLayout};

const MEMORY_END_INCLUSIVE: usize = 0xFFFFFFFF;
pub type DefaultAddrSpace = AddressSpace<{ MEMORY_END_INCLUSIVE + 1 }>;

pub fn default_layout() -> impl VirtualMemoryLayout {
    SimpleMemoryLayout::new(
        MEMORY_END_INCLUSIVE,
        [
            TranslationDescriptor {
                name: "Kernel code and RO data",
                virtual_range: rx_range_inclusive,
                physical_range_translation: Translation::Identity,
                attribute_fields: AttributeFields {
                    mem_attributes: MemAttributes::CacheableDRAM,
                    acc_perms: AccessPermissions::ReadOnly,
                    execute_never: false,
                },
            },
            TranslationDescriptor {
                name: "Device MMIO",
                virtual_range: mmio_range_inclusive,
                physical_range_translation: Translation::Identity,
                attribute_fields: AttributeFields {
                    mem_attributes: MemAttributes::Device,
                    acc_perms: AccessPermissions::ReadWrite,
                    execute_never: true,
                },
            },
        ],
    )
}

fn rx_range_inclusive() -> RangeInclusive<usize> {
    // Notice the subtraction to turn the exclusive end into an inclusive end.
    #[allow(clippy::range_minus_one)]
    RangeInclusive::new(rx_start(), rx_end_exclusive() - 1)
}

fn mmio_range_inclusive() -> RangeInclusive<usize> {
    // Todo unhardcode
    RangeInclusive::new(0xFE00_0000, 0xFF84_FFFF)
}

// Symbols from the linker script.
extern "Rust" {
    static __rx_start: UnsafeCell<()>;
    static __rx_end_exclusive: UnsafeCell<()>;
}

/// Start address of the Read+Execute (RX) range.
#[inline(always)]
fn rx_start() -> usize {
    unsafe { __rx_start.get() as usize }
}

/// Exclusive end address of the Read+Execute (RX) range.
#[inline(always)]
fn rx_end_exclusive() -> usize {
    unsafe { __rx_end_exclusive.get() as usize }
}
