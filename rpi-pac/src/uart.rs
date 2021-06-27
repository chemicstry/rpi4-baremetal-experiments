use register::{
    mmio::{ReadOnly, ReadWrite, WriteOnly},
    register_bitfields, register_structs,
};

// PL011 UART registers.
//
// Descriptions taken from "PrimeCell UART (PL011) Technical Reference Manual" r1p5.
register_bitfields! {
    u32,

    /// Data Register
    pub DR [
        /// Overrun error. This bit is set to 1 if data is received and the receive FIFO is already full.
        ///
        /// This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it.
        OE OFFSET(11) NUMBITS(1) [],
        /// Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input
        /// was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop
        /// bits).
        ///
        /// In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs,
        /// only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data
        /// input goes to a 1 (marking state), and the next valid start bit is received.
        BE OFFSET(10) NUMBITS(1) [],
        /// Parity error. When set to 1, it indicates that the parity of the received data character does not match the
        /// parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H on page 3-12 select.
        ///
        /// In FIFO mode, this error is associated with the character at the top of the FIFO.
        PE OFFSET(9) NUMBITS(1) [],
        /// Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid
        /// stop bit is 1).
        ///
        /// In FIFO mode, this error is associated with the character at the top of the FIFO.
        FE OFFSET(8) NUMBITS(1) [],
        /// Receive (read) data character.
        /// Transmit (write) data character.
        DATA OFFSET(0) NUMBITS(7) []
    ],

    /// Receive Status Register/Error Clear Register
    pub RSR_ECR [
        /// Overrun error. This bit is set to 1 if data is received and the receive FIFO is already full.
        ///
        /// This bit is cleared to 0 by a write to UARTECR.
        ///
        /// The FIFO contents remain valid because no more data is written when the FIFO is full, only the contents
        /// of the shift register are overwritten. The CPU must now read the data, to empty the FIFO.
        OE OFFSET(3) NUMBITS(1) [],
        /// Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input
        /// was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop
        /// bits).
        ///
        /// This bit is cleared to 0 after a write to UARTECR.
        ///
        /// In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs,
        /// only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data
        /// input goes to a 1 (marking state), and the next valid start bit is received.
        BE OFFSET(2) NUMBITS(1) [],
        /// Parity error. When set to 1, it indicates that the parity of the received data character does not match the
        /// parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H on page 3-12 select.
        ///
        /// This bit is cleared to 0 by a write to UARTECR.
        ///
        /// In FIFO mode, this error is associated with the character at the top of the FIFO.
        PE OFFSET(1) NUMBITS(1) [],
        /// Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid
        /// stop bit is 1).
        ///
        /// This bit is cleared to 0 by a write to UARTECR.
        ///
        /// In FIFO mode, this error is associated with the character at the top of the FIFO.
        FE OFFSET(0) NUMBITS(1) []
    ],

    /// Flag Register.
    pub FR [
        /// Transmit FIFO empty. The meaning of this bit depends on the state of the FEN bit in the
        /// Line Control Register, LCR_H.
        ///
        /// - If the FIFO is disabled, this bit is set when the transmit holding register is empty.
        /// - If the FIFO is enabled, the TXFE bit is set when the transmit FIFO is empty.
        /// - This bit does not indicate if there is data in the transmit shift register.
        TXFE OFFSET(7) NUMBITS(1) [],

        /// Transmit FIFO full. The meaning of this bit depends on the state of the FEN bit in the
        /// LCR_H Register.
        ///
        /// - If the FIFO is disabled, this bit is set when the transmit holding register is full.
        /// - If the FIFO is enabled, the TXFF bit is set when the transmit FIFO is full.
        TXFF OFFSET(5) NUMBITS(1) [],

        /// Receive FIFO empty. The meaning of this bit depends on the state of the FEN bit in the
        /// LCR_H Register.
        ///
        /// If the FIFO is disabled, this bit is set when the receive holding register is empty. If
        /// the FIFO is enabled, the RXFE bit is set when the receive FIFO is empty.

        /// Receive FIFO empty. The meaning of this bit depends on the state of the FEN bit in the
        /// LCR_H Register.
        ///
        /// - If the FIFO is disabled, this bit is set when the receive holding register is empty.
        /// - If the FIFO is enabled, the RXFE bit is set when the receive FIFO is empty.
        RXFE OFFSET(4) NUMBITS(1) [],

        /// UART busy. If this bit is set to 1, the UART is busy transmitting data. This bit remains
        /// set until the complete byte, including all the stop bits, has been sent from the shift
        /// register.
        ///
        /// This bit is set as soon as the transmit FIFO becomes non-empty, regardless of whether
        /// the UART is enabled or not.
        BUSY OFFSET(3) NUMBITS(1) []
    ],

    /// Integer Baud Rate Divisor.
    pub IBRD [
        /// The integer baud rate divisor.
        BAUD_DIVINT OFFSET(0) NUMBITS(16) []
    ],

    /// Fractional Baud Rate Divisor.
    pub FBRD [
        ///  The fractional baud rate divisor.
        BAUD_DIVFRAC OFFSET(0) NUMBITS(6) []
    ],

    /// Line Control Register.
    pub LCR_H [
        /// Stick parity select.
        ///
        /// This bit has no effect when the PEN bit disables parity checking and generation. See Table 3-11 on
        /// page 3-14 for the parity truth table
        SPS OFFSET(7) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Word length. These bits indicate the number of data bits transmitted or received in a
        /// frame.
        WLEN OFFSET(5) NUMBITS(2) [
            FiveBit = 0b00,
            SixBit = 0b01,
            SevenBit = 0b10,
            EightBit = 0b11
        ],

        /// Enable FIFOs:
        ///
        /// 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding
        /// registers.
        ///
        /// 1 = Transmit and receive FIFO buffers are enabled (FIFO mode).
        FEN  OFFSET(4) NUMBITS(1) [
            FifosDisabled = 0,
            FifosEnabled = 1
        ],

        /// Two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive
        /// logic does not check for two stop bits being received.
        STP2 OFFSET(3) NUMBITS(1) [
            OneBit = 0,
            TwoBits = 1
        ],

        /// Even parity select. Controls the type of parity the UART uses during transmission and reception:
        /// This bit has no effect when the PEN bit disables parity checking and generation. See Table 3-11 on
        /// page 3-14 for the parity truth table.
        EPS OFFSET(2) NUMBITS(1) [
            OddParity = 0,
            EvenParity = 1
        ],

        /// Parity enable.
        /// See Table 3-11 on page 3-14 for the parity truth table
        PEN OFFSET(1) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        ///  Send break. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after
        /// completing transmission of the current character. For the proper execution of the break command, the
        /// software must set this bit for at least two complete frames.
        /// For normal use, this bit must be cleared to 0.
        BRK OFFSET(0) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ]
    ],

    /// Control Register.
    pub CR [
        /// Receive enable. If this bit is set to 1, the receive section of the UART is enabled.
        /// Data reception occurs for either UART signals or SIR signals depending on the setting of
        /// the SIREN bit. When the UART is disabled in the middle of reception, it completes the
        /// current character before stopping.
        RXE OFFSET(9) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Transmit enable. If this bit is set to 1, the transmit section of the UART is enabled.
        /// Data transmission occurs for either UART signals, or SIR signals depending on the
        /// setting of the SIREN bit. When the UART is disabled in the middle of transmission, it
        /// completes the current character before stopping.
        TXE OFFSET(8) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// UART enable:
        ///
        /// 0 = UART is disabled. If the UART is disabled in the middle of transmission or
        /// reception, it completes the current character before stopping.
        ///
        /// 1 = The UART is enabled. Data transmission and reception occurs for either UART signals
        /// or SIR signals depending on the setting of the SIREN bit
        UARTEN OFFSET(0) NUMBITS(1) [
            /// If the UART is disabled in the middle of transmission or reception, it completes the
            /// current character before stopping.
            Disabled = 0,
            Enabled = 1
        ]
    ],

    /// Interrupt Clear Register.
    pub ICR [
        /// Meta field for all pending interrupts.
        ALL OFFSET(0) NUMBITS(11) []
    ]
}

register_structs! {
    pub RegisterBlock {
        (0x00 => pub dr: ReadWrite<u32, DR::Register>),
        (0x04 => pub rsr_ecr: ReadWrite<u32, RSR_ECR::Register>),
        (0x08 => _reserved1),
        (0x18 => pub fr: ReadOnly<u32, FR::Register>),
        (0x1c => _reserved2),
        (0x24 => pub ibrd: WriteOnly<u32, IBRD::Register>),
        (0x28 => pub fbrd: WriteOnly<u32, FBRD::Register>),
        (0x2c => pub lcr_h: WriteOnly<u32, LCR_H::Register>),
        (0x30 => pub cr: WriteOnly<u32, CR::Register>),
        (0x34 => _reserved3),
        (0x44 => pub icr: WriteOnly<u32, ICR::Register>),
        (0x48 => @END),
    }
}
