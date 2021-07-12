use core::fmt;

use crate::mmu::{AttributeFields, Translation, TranslationDescriptor};

use super::VirtualMemoryLayout;

/// Memory layout which default to identity mapped normal cacheable DRAM, except for what is defined in special regions
pub struct SimpleMemoryLayout<const NUM_SPECIAL_RANGES: usize> {
    /// The last (inclusive) address of the address space.
    max_virt_addr_inclusive: usize,

    /// Array of descriptors for non-standard (normal cacheable DRAM) memory regions.
    inner: [TranslationDescriptor; NUM_SPECIAL_RANGES],
}

impl<const NUM_SPECIAL_RANGES: usize> SimpleMemoryLayout<{ NUM_SPECIAL_RANGES }> {
    /// Create a new instance.
    pub const fn new(max: usize, layout: [TranslationDescriptor; NUM_SPECIAL_RANGES]) -> Self {
        Self {
            max_virt_addr_inclusive: max,
            inner: layout,
        }
    }
}

impl<const NUM_SPECIAL_RANGES: usize> VirtualMemoryLayout
    for SimpleMemoryLayout<{ NUM_SPECIAL_RANGES }>
{
    /// If the address is not found in `inner`, return an identity mapped default with normal
    /// cacheable DRAM attributes.
    fn virt_addr_properties(
        &self,
        virt_addr: usize,
    ) -> Result<(usize, AttributeFields), &'static str> {
        if virt_addr > self.max_virt_addr_inclusive {
            return Err("Address out of range");
        }

        for i in self.inner.iter() {
            if (i.virtual_range)().contains(&virt_addr) {
                let output_addr = match i.physical_range_translation {
                    Translation::Identity => virt_addr,
                    Translation::Offset(a) => a + (virt_addr - (i.virtual_range)().start()),
                };

                return Ok((output_addr, i.attribute_fields));
            }
        }

        Ok((virt_addr, AttributeFields::default()))
    }
}

impl<const NUM_SPECIAL_RANGES: usize> fmt::Display for SimpleMemoryLayout<{ NUM_SPECIAL_RANGES }> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in self.inner.iter() {
            writeln!(f, "{}", i)?;
        }

        Ok(())
    }
}
