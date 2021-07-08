use rpi_pac::gicv2::gicd::*;

#[derive(Debug, Clone, Copy)]
pub struct CoreMask(u8);

#[derive(Debug, Clone, Copy)]
pub struct IrqNumber(u32);

impl IrqNumber {
    /// TODO: add limits
    pub fn new(num: u32) -> Self {
        Self(num)
    }

    /// Returns true if IRQ is private to the executing core
    pub fn is_private(&self) -> bool {
        // Fisrt 32 IRQs are private
        self.0 <= 31
    }

    /// Returns true if it's Software Generated Interrupt (SGI)
    pub fn is_sgi(&self) -> bool {
        self.0 <= 15
    }
}

/// Interface to the GIC Distributor peripheral
pub struct Gicd {
    gicd: rpi_pac::GicdShared,
}

impl Gicd {
    pub fn new(gicd: rpi_pac::GicdShared) -> Self {
        Self { gicd }
    }

    pub fn enable_irq(&mut self, irq_num: IrqNumber) {
        if irq_num.is_private() {
            panic!("Attempted to enable a private IRQ on shared distributor");
        }

        // Each bit in the u32 enable register corresponds to one IRQ number. Shift right by 5
        // (division by 32) and arrive at the index for the respective ISENABLER[i].
        // Subtract 1 because first ISENABLER register is in the banked set.
        let enable_reg_index = (irq_num.0 >> 5) - 1;
        let enable_bit: u32 = 1u32 << (irq_num.0 % 32);

        let reg = &self.gicd.isenabler[enable_reg_index as usize];
        reg.set(reg.get() | enable_bit);
    }

    /// Sets CoreMask for all interrupts
    pub fn set_global_core_mask(&mut self, mask: CoreMask) {
        let mask = mask.0 as u32;

        for reg in &self.gicd.itargetsr {
            reg.write(
                ITARGETSR::Offset0.val(mask)
                    + ITARGETSR::Offset1.val(mask)
                    + ITARGETSR::Offset2.val(mask)
                    + ITARGETSR::Offset3.val(mask),
            );
        }
    }
}

/// Interface to the core-local parts of GIC Distributor
pub struct GicdLocal {
    gicd: rpi_pac::GicdBanked,
}

/// Software Generated Interrupt target
pub enum SgiTarget {
    Mask(CoreMask),
    AllExceptCurrent,
    OnlyCurrent,
}

impl GicdLocal {
    pub fn new(gicd: rpi_pac::GicdBanked) -> Self {
        Self { gicd }
    }

    pub fn enable_irq(&mut self, irq_num: IrqNumber) {
        if !irq_num.is_private() {
            panic!("Attempted to enable a shared IRQ on a private distributor");
        }

        let enable_bit: u32 = 1u32 << (irq_num.0 % 32);

        let reg = &self.gicd.isenabler;
        reg.set(reg.get() | enable_bit);
    }

    /// Returns core mask of currently executing core
    pub fn core_mask(&mut self) -> CoreMask {
        // ITARGETSR registers 0-7 are read only and return mask of the current core
        CoreMask(self.gicd.itargetsr[0].read(ITARGETSR::Offset0) as u8)
    }

    pub fn pend_sgi(&mut self, irq_num: IrqNumber, target: SgiTarget) {
        if !irq_num.is_sgi() {
            panic!("Attempted to pend IRQ {:?}, which is not SGI!", irq_num);
        }

        let (target_list_filter, mask) = match target {
            SgiTarget::Mask(mask) => (SGIR::TargetListFilter::CPUTargetList, mask),
            SgiTarget::AllExceptCurrent => (SGIR::TargetListFilter::AllExceptCurrent, CoreMask(0)),
            SgiTarget::OnlyCurrent => (SGIR::TargetListFilter::OnlyCurrent, CoreMask(0)),
        };

        self.gicd.sgir.write(
            target_list_filter
                + SGIR::CPUTargetList.val(mask.0 as u32)
                + SGIR::SGIINTID.val(irq_num.0),
        );
    }
}
