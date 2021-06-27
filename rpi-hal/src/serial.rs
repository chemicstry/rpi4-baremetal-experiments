use rpi_pac::{uart::*, Uart0, Uart2, Uart3, Uart4, Uart5};

use crate::{
    gpio::{typestate::*, GpioExt, Pin},
    time::{Bps, Hertz},
};

pub mod config {
    use crate::time::Bps;
    use crate::time::U32Ext;

    pub enum WordLength {
        DataBits8,
        DataBits7,
        DataBits6,
        DataBits5,
    }

    pub enum Parity {
        ParityNone,
        ParityEven,
        ParityOdd,
    }

    pub enum StopBits {
        #[doc = "1 stop bit"]
        STOP1,
        #[doc = "2 stop bits"]
        STOP2,
    }

    pub enum FifoConfig {
        Disabled,
        Enabled,
    }

    pub struct Config {
        pub baudrate: Bps,
        pub wordlength: WordLength,
        pub parity: Parity,
        pub stopbits: StopBits,
        pub fifo: FifoConfig,
    }

    impl Config {
        pub fn baudrate(mut self, baudrate: Bps) -> Self {
            self.baudrate = baudrate;
            self
        }

        pub fn parity_none(mut self) -> Self {
            self.parity = Parity::ParityNone;
            self
        }

        pub fn parity_even(mut self) -> Self {
            self.parity = Parity::ParityEven;
            self
        }

        pub fn parity_odd(mut self) -> Self {
            self.parity = Parity::ParityOdd;
            self
        }

        pub fn wordlength_8(mut self) -> Self {
            self.wordlength = WordLength::DataBits8;
            self
        }

        pub fn wordlength_7(mut self) -> Self {
            self.wordlength = WordLength::DataBits7;
            self
        }

        pub fn wordlength_6(mut self) -> Self {
            self.wordlength = WordLength::DataBits6;
            self
        }

        pub fn wordlength_5(mut self) -> Self {
            self.wordlength = WordLength::DataBits5;
            self
        }

        pub fn stopbits(mut self, stopbits: StopBits) -> Self {
            self.stopbits = stopbits;
            self
        }
    }

    impl Default for Config {
        fn default() -> Config {
            let baudrate = 19_200_u32.bps();
            Config {
                baudrate,
                wordlength: WordLength::DataBits8,
                parity: Parity::ParityNone,
                stopbits: StopBits::STOP1,
                fifo: FifoConfig::Enabled,
            }
        }
    }
}

pub trait Pins<UART> {}
pub trait PinTx<UART> {}
pub trait PinRx<UART> {}

impl<UART, TX, RX> Pins<UART> for (TX, RX)
where
    TX: PinTx<UART>,
    RX: PinRx<UART>,
{
}

/// A filler type for when the Tx pin is unnecessary
pub struct NoTx;
/// A filler type for when the Rx pin is unnecessary
pub struct NoRx;

impl PinTx<Uart0> for NoTx {}
impl PinRx<Uart0> for NoRx {}

impl PinTx<Uart2> for NoTx {}
impl PinRx<Uart2> for NoRx {}

impl PinTx<Uart3> for NoTx {}
impl PinRx<Uart3> for NoRx {}

impl PinTx<Uart4> for NoTx {}
impl PinRx<Uart4> for NoRx {}

impl PinTx<Uart5> for NoTx {}
impl PinRx<Uart5> for NoRx {}

impl<'a, GPIO: GpioExt> PinTx<Uart0> for Pin<'a, GPIO, AltFunc0, Floating, 14> {}
impl<'a, GPIO: GpioExt> PinRx<Uart0> for Pin<'a, GPIO, AltFunc0, Floating, 15> {}

impl<'a, GPIO: GpioExt> PinTx<Uart0> for Pin<'a, GPIO, AltFunc3, Floating, 32> {}
impl<'a, GPIO: GpioExt> PinRx<Uart0> for Pin<'a, GPIO, AltFunc3, Floating, 33> {}

impl<'a, GPIO: GpioExt> PinTx<Uart0> for Pin<'a, GPIO, AltFunc2, Floating, 36> {}
impl<'a, GPIO: GpioExt> PinRx<Uart0> for Pin<'a, GPIO, AltFunc2, Floating, 37> {}

impl<'a, GPIO: GpioExt> PinTx<Uart2> for Pin<'a, GPIO, AltFunc4, Floating, 0> {}
impl<'a, GPIO: GpioExt> PinRx<Uart2> for Pin<'a, GPIO, AltFunc4, Floating, 1> {}

impl<'a, GPIO: GpioExt> PinTx<Uart3> for Pin<'a, GPIO, AltFunc4, Floating, 4> {}
impl<'a, GPIO: GpioExt> PinRx<Uart3> for Pin<'a, GPIO, AltFunc4, Floating, 5> {}

impl<'a, GPIO: GpioExt> PinTx<Uart4> for Pin<'a, GPIO, AltFunc4, Floating, 8> {}
impl<'a, GPIO: GpioExt> PinRx<Uart4> for Pin<'a, GPIO, AltFunc4, Floating, 9> {}

impl<'a, GPIO: GpioExt> PinTx<Uart5> for Pin<'a, GPIO, AltFunc4, Floating, 12> {}
impl<'a, GPIO: GpioExt> PinRx<Uart5> for Pin<'a, GPIO, AltFunc4, Floating, 13> {}

/// Serial abstraction
pub struct Serial<UART, PINS> {
    uart: UART,
    _pins: PINS,
}

pub trait Instance: core::ops::Deref<Target = rpi_pac::uart::RegisterBlock> {}
impl<T> Instance for T where T: core::ops::Deref<Target = rpi_pac::uart::RegisterBlock> {}

pub struct BaudRateDivisor {
    ibrd: u32,
    fbrd: u32,
}

impl BaudRateDivisor {
    /// Calculates divisor from UART peripheral clock and desired baudrate.
    /// UART peripheral clock is set by clock manager, which is undocumented.
    /// UART0 clock can be specified in SD card config.txt (init_uart_clock).
    pub fn new(uart_freq: Hertz, baudrate: Bps) -> Self {
        // Baud rate divisor is calculated as `UART_FREQ / (16 * BAUDRATE)`.
        // Then, integer value is put into IBRD register and FBRD is calculated as `floor(FRACTIONAL_PART * 64 + 0.5)`.
        let base_freq = uart_freq.0 / 16;
        // Calculate the integer part
        let ibrd = base_freq / baudrate.0;
        // Do some math to avoid floating point calculation
        let remainder = base_freq % baudrate.0;
        let fbrd = (remainder * 128 + 1) / (2 * baudrate.0);

        Self { ibrd, fbrd }
    }
}

#[derive(Debug)]
pub enum SerialError {
    /// Data was received when FIFO was already full.
    Overrun,
    /// Break condition was detected, indicating that the received data input
    /// was held LOW for longer than a full-word transmission time (defined as start, data, parity, and stop bits).
    Break,
    /// Parity of the received data did not match the configured parity check.
    Parity,
    /// Received character did not have a valid stop bit.
    Framing,
}

pub type Result<T> = nb::Result<T, SerialError>;

impl<UART, PINS> Serial<UART, PINS>
where
    PINS: Pins<UART>,
    UART: Instance,
{
    pub fn new(uart: UART, pins: PINS, config: config::Config, uart_freq: Hertz) -> Self {
        // Turn the UART off temporarily.
        uart.cr.set(0);

        // Clear all pending interrupts.
        uart.icr.write(ICR::ALL::CLEAR);

        let brd = BaudRateDivisor::new(uart_freq, config.baudrate);

        let wlen = match config.wordlength {
            config::WordLength::DataBits8 => LCR_H::WLEN::EightBit,
            config::WordLength::DataBits7 => LCR_H::WLEN::SevenBit,
            config::WordLength::DataBits6 => LCR_H::WLEN::SixBit,
            config::WordLength::DataBits5 => LCR_H::WLEN::FiveBit,
        };

        let eps = match config.parity {
            config::Parity::ParityNone | config::Parity::ParityEven => LCR_H::EPS::EvenParity,
            config::Parity::ParityOdd => LCR_H::EPS::OddParity,
        };

        let pen = match config.parity {
            config::Parity::ParityNone => LCR_H::PEN::Disabled,
            config::Parity::ParityEven | config::Parity::ParityOdd => LCR_H::PEN::Enabled,
        };

        let stp = match config.stopbits {
            config::StopBits::STOP1 => LCR_H::STP2::OneBit,
            config::StopBits::STOP2 => LCR_H::STP2::TwoBits,
        };

        let fen = match config.fifo {
            config::FifoConfig::Disabled => LCR_H::FEN::FifosDisabled,
            config::FifoConfig::Enabled => LCR_H::FEN::FifosEnabled,
        };

        // From the PL011 Technical Reference Manual:
        //
        // The LCR_H, IBRD, and FBRD registers form the single 30-bit wide LCR Register that is
        // updated on a single write strobe generated by a LCR_H write. So, to internally update the
        // contents of IBRD or FBRD, a LCR_H write must always be performed at the end.
        //
        // Set the baud rate, 8N1 and FIFO enabled.
        uart.ibrd.write(IBRD::BAUD_DIVINT.val(brd.ibrd));
        uart.fbrd.write(FBRD::BAUD_DIVFRAC.val(brd.fbrd));
        uart.lcr_h
            .write(LCR_H::SPS::Disabled + wlen + fen + stp + eps + pen + LCR_H::BRK::Disabled);

        // Turn the UART on.
        uart.cr
            .write(CR::UARTEN::Enabled + CR::TXE::Enabled + CR::RXE::Enabled);

        Self { uart, _pins: pins }
    }

    /// Send a character.
    pub fn write(&mut self, b: u8) -> Result<()> {
        if self.uart.fr.matches_all(FR::TXFF::SET) {
            return Err(nb::Error::WouldBlock);
        }

        // Write the character to the buffer.
        self.uart.dr.write(DR::DATA.val(b as u32));

        Ok(())
    }

    /// Block execution until the last buffered character has been physically put on the TX wire.
    pub fn flush(&mut self) -> Result<()> {
        // Check if busy
        if self.uart.fr.matches_all(FR::BUSY::SET) {
            return Err(nb::Error::WouldBlock);
        }

        Ok(())
    }

    /// Retrieve a character.
    pub fn read(&mut self) -> Result<u8> {
        let rsr = self.uart.rsr_ecr.extract();

        if rsr.is_set(RSR_ECR::OE) {
            self.uart.rsr_ecr.set(0);
            return Err(nb::Error::Other(SerialError::Overrun));
        } else if rsr.is_set(RSR_ECR::BE) {
            self.uart.rsr_ecr.set(0);
            return Err(nb::Error::Other(SerialError::Break));
        } else if rsr.is_set(RSR_ECR::PE) {
            self.uart.rsr_ecr.set(0);
            return Err(nb::Error::Other(SerialError::Parity));
        } else if rsr.is_set(RSR_ECR::FE) {
            self.uart.rsr_ecr.set(0);
            return Err(nb::Error::Other(SerialError::Framing));
        } else if self.uart.fr.matches_all(FR::RXFE::SET) {
            // RX FIFO is empty
            return Err(nb::Error::WouldBlock);
        } else {
            Ok(self.uart.dr.read(DR::DATA) as u8)
        }
    }
}

impl<UART, PINS> embedded_hal::serial::Read<u8> for Serial<UART, PINS>
where
    PINS: Pins<UART>,
    UART: Instance,
{
    type Error = SerialError;

    fn read(&mut self) -> Result<u8> {
        Serial::read(self)
    }
}

impl<UART, PINS> embedded_hal::serial::Write<u8> for Serial<UART, PINS>
where
    PINS: Pins<UART>,
    UART: Instance,
{
    type Error = SerialError;

    fn write(&mut self, word: u8) -> Result<()> {
        Serial::write(self, word)
    }

    fn flush(&mut self) -> Result<()> {
        Serial::flush(self)
    }
}
