use super::AttributeFields;

pub mod default;
pub mod simple;

pub trait VirtualMemoryLayout {
    /// For a virtual address, find and return the physical output address and corresponding
    /// attributes.
    fn virt_addr_properties(
        &self,
        virt_addr: usize,
    ) -> Result<(usize, AttributeFields), &'static str>;
}
