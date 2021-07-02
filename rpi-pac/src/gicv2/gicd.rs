use register::{mmio::{ReadOnly, ReadWrite}, register_bitfields, register_structs};

register_bitfields! {
    u32,

    /// Distributor Control Register
    pub CTLR [
        Enable OFFSET(0) NUMBITS(1) []
    ],

    /// Interrupt Controller Type Register
    pub TYPER [
        ITLinesNumber OFFSET(0)  NUMBITS(5) []
    ],

    /// Interrupt Processor Targets Registers
    pub ITARGETSR [
        Offset3 OFFSET(24) NUMBITS(8) [],
        Offset2 OFFSET(16) NUMBITS(8) [],
        Offset1 OFFSET(8)  NUMBITS(8) [],
        Offset0 OFFSET(0)  NUMBITS(8) []
    ]
}

register_structs! {
    pub SharedRegisterBlock {
        (0x000 => pub ctlr: ReadWrite<u32, CTLR::Register>),
        (0x004 => pub typer: ReadOnly<u32, TYPER::Register>),
        (0x008 => _reserved1),
        (0x104 => pub isenabler: [ReadWrite<u32>; 31]),
        (0x108 => _reserved2),
        (0x820 => pub itargetsr: [ReadWrite<u32, ITARGETSR::Register>; 248]),
        (0x824 => @END),
    }
}

register_structs! {
    pub BankedRegisterBlock {
        (0x000 => _reserved1),
        (0x100 => pub isenabler: ReadWrite<u32>),
        (0x104 => _reserved2),
        (0x800 => pub itargetsr: [ReadOnly<u32, ITARGETSR::Register>; 8]),
        (0x804 => @END),
    }
}
