use register::{
    mmio::{ReadOnly, ReadWrite, WriteOnly},
    register_bitfields, register_structs,
};

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
    ],

    /// Software Generated Interrupt Register
    pub SGIR [
        /// Determines how the distributor must process the requested SGI
        TargetListFilter    OFFSET(24) NUMBITS(2) [
            /// Forward the interrupt to the CPU interfaces specified in the CPUTargetList field.
            CPUTargetList       = 0b00,
            /// Forward the interrupt to all CPU interfaces except that of the processor that requested the interrupt.
            AllExceptCurrent    = 0b01,
            /// Forward the interrupt only to the CPU interface of the processor that requested the interrupt.
            OnlyCurrent         = 0b10
        ],
        /// When TargetList Filter = 0b00, defines the CPU interfaces to which the Distributor must forward the interrupt.
        ///
        /// Each bit of CPUTargetList[7:0] refers to the corresponding CPU interface, for example
        /// CPUTargetList[0] corresponds to CPU interface 0. Setting a bit to 1 indicates that the interrupt must be
        /// forwarded to the corresponding interface.
        ///
        /// If this field is 0x00 when TargetListFilter is 0b00, the Distributor does not forward the interrupt to any
        /// CPU interface.
        CPUTargetList       OFFSET(16) NUMBITS(8) [],
        /// Implemented only if the GIC includes the Security Extensions.
        ///
        /// Specifies the required security value of the SGI:
        /// - 0 Forward the SGI specified in the SGIINTID field to a specified CPU interface only if the
        ///   SGI is configured as Group 0 on that interface.
        /// - 1 Forward the SGI specified in the SGIINTID field to a specified CPU interfaces only if
        ///   the SGI is configured as Group 1 on that interface.
        ///
        /// This field is writable only by a Secure access. Any Non-secure write to the GICD_SGIR generates an
        /// SGI only if the specified SGI is programmed as Group 1, regardless of the value of bit[15] of the write.
        NSATT               OFFSET(15) NUMBITS(1) [],
        /// The Interrupt ID of the SGI to forward to the specified CPU interfaces.
        SGIINTID            OFFSET(0)  NUMBITS(4) []
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
        (0x820 => _reserved3),
        (0xF00 => pub sgir: WriteOnly<u32, SGIR::Register>),
        (0xF04 => _reserved4),
        (0xF20 => pub spendsgir: ReadWrite<u32>),
        (0xF24 => @END),
    }
}
