// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2021 Andre Richter <andre.o.richter@gmail.com>

//! PL011 UART driver.
//!
//! # Resources
//!
//! - <https://github.com/raspberrypi/documentation/files/1888662/BCM2837-ARM-Peripherals.-.Revised.-.V2-1.pdf>
//! - <https://developer.arm.com/documentation/ddi0183/latest>

use embedded_hal as hal;
use register::{mmio::*, register_bitfields, register_structs};

// PL011 UART registers.
//
// Descriptions taken from "PrimeCell UART (PL011) Technical Reference Manual" r1p5.
register_bitfields! {
    u32,

    /// Flag Register.
    FR [
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
    IBRD [
        /// The integer baud rate divisor.
        BAUD_DIVINT OFFSET(0) NUMBITS(16) []
    ],

    /// Fractional Baud Rate Divisor.
    FBRD [
        ///  The fractional baud rate divisor.
        BAUD_DIVFRAC OFFSET(0) NUMBITS(6) []
    ],

    /// Line Control Register.
    LCR_H [
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
        ]
    ],

    /// Control Register.
    CR [
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
    ICR [
        /// Meta field for all pending interrupts.
        ALL OFFSET(0) NUMBITS(11) []
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x00 => DR: ReadWrite<u32>),
        (0x04 => _reserved1),
        (0x18 => FR: ReadOnly<u32, FR::Register>),
        (0x1c => _reserved2),
        (0x24 => IBRD: WriteOnly<u32, IBRD::Register>),
        (0x28 => FBRD: WriteOnly<u32, FBRD::Register>),
        (0x2c => LCR_H: WriteOnly<u32, LCR_H::Register>),
        (0x30 => CR: WriteOnly<u32, CR::Register>),
        (0x34 => _reserved3),
        (0x44 => ICR: WriteOnly<u32, ICR::Register>),
        (0x48 => @END),
    }
}

pub struct PL011Uart {
    base_addr: usize,
}

impl core::ops::Deref for PL011Uart {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

#[derive(Debug)]
pub enum PL011UartError {}

pub type Result<T> = nb::Result<T, PL011UartError>;

impl PL011Uart {
    /// Create an instance.
    ///
    /// # Safety
    ///
    /// - The user must ensure to provide a correct MMIO start address.
    pub const unsafe fn new(base_addr: usize) -> Self {
        Self { base_addr }
    }

    fn ptr(&self) -> *const RegisterBlock {
        self.base_addr as *const _
    }

    /// Set up baud rate and characteristics.
    ///
    /// This results in 8N1 and 921_600 baud.
    ///
    /// The calculation for the BRD is (we set the clock to 48 MHz in config.txt):
    /// `(48_000_000 / 16) / 921_600 = 3.2552083`.
    ///
    /// This means the integer part is `3` and goes into the `IBRD`.
    /// The fractional part is `0.2552083`.
    ///
    /// `FBRD` calculation according to the PL011 Technical Reference Manual:
    /// `INTEGER((0.2552083 * 64) + 0.5) = 16`.
    ///
    /// Therefore, the generated baud rate divider is: `3 + 16/64 = 3.25`. Which results in a
    /// genrated baud rate of `48_000_000 / (16 * 3.25) = 923_077`.
    ///
    /// Error = `((923_077 - 921_600) / 921_600) * 100 = 0.16%`.
    pub fn init(&mut self) {
        // Execution can arrive here while there are still characters queued in the TX FIFO and
        // actively being sent out by the UART hardware. If the UART is turned off in this case,
        // those queued characters would be lost.
        //
        // For example, this can happen during runtime on a call to panic!(), because panic!()
        // initializes its own UART instance and calls init().
        //
        // Hence, flush first to ensure all pending characters are transmitted.
        nb::block!(self.flush_tx()).unwrap();

        // Turn the UART off temporarily.
        self.CR.set(0);

        // Clear all pending interrupts.
        self.ICR.write(ICR::ALL::CLEAR);

        // From the PL011 Technical Reference Manual:
        //
        // The LCR_H, IBRD, and FBRD registers form the single 30-bit wide LCR Register that is
        // updated on a single write strobe generated by a LCR_H write. So, to internally update the
        // contents of IBRD or FBRD, a LCR_H write must always be performed at the end.
        //
        // Set the baud rate, 8N1 and FIFO enabled.
        self.IBRD.write(IBRD::BAUD_DIVINT.val(3));
        self.FBRD.write(FBRD::BAUD_DIVFRAC.val(16));
        self.LCR_H
            .write(LCR_H::WLEN::EightBit + LCR_H::FEN::FifosEnabled);

        // Turn the UART on.
        self.CR
            .write(CR::UARTEN::Enabled + CR::TXE::Enabled + CR::RXE::Enabled);
    }

    /// Send a character.
    pub fn write_byte(&mut self, b: u8) -> Result<()> {
        if self.FR.matches_all(FR::TXFF::SET) {
            return Err(nb::Error::WouldBlock);
        }

        // Write the character to the buffer.
        self.DR.set(b as u32);

        Ok(())
    }

    /// Block execution until the last buffered character has been physically put on the TX wire.
    pub fn flush_tx(&self) -> Result<()> {
        // Check if busy
        if self.FR.matches_all(FR::BUSY::SET) {
            return Err(nb::Error::WouldBlock);
        }

        Ok(())
    }

    /// Retrieve a character.
    pub fn read_byte(&mut self) -> Result<u8> {
        // If RX FIFO is empty,
        if self.FR.matches_all(FR::RXFE::SET) {
            return Err(nb::Error::WouldBlock);
        }

        // Read one character.
        Ok(self.DR.get() as u8)
    }
}

impl hal::serial::Write<u8> for PL011Uart {
    type Error = PL011UartError;

    fn write(&mut self, byte: u8) -> Result<()> {
        self.write_byte(byte)
    }

    fn flush(&mut self) -> Result<()> {
        self.flush_tx()
    }
}

impl hal::serial::Read<u8> for PL011Uart {
    type Error = PL011UartError;

    fn read(&mut self) -> Result<u8> {
        self.read_byte()
    }
}
