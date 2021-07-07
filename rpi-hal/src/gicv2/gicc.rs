use rpi_pac::gicv2::gicc::*;

pub struct Gicc {
    gicc: rpi_pac::Gicc,
}

impl Gicc {
    pub fn new(gicc: rpi_pac::Gicc) -> Self {
        Self { gicc }
    }

    pub fn enable(&mut self) {
        self.gicc.ctlr.write(CTLR::Enable::SET);
    }

    pub fn disable(&mut self) {
        self.gicc.ctlr.write(CTLR::Enable::CLEAR);
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.gicc.pmr.write(PMR::Priority.val(priority as u32));
    }

    /// Gets the number of the highest priority pending IRQ.
    ///
    /// # Safety
    ///
    /// - Must only be accessed from IRQ context
    pub unsafe fn pending_irq(&mut self) -> u32 {
        self.gicc.iar.read(IAR::InterruptID)
    }

    /// Complete handling of the currently active IRQ.
    ///
    /// # Safety
    ///
    /// - Must only be accessed from IRQ context
    pub unsafe fn mark_completed(&mut self, irq_num: u32) {
        self.gicc.eoir.write(EOIR::EOIINTID.val(irq_num));
    }
}
