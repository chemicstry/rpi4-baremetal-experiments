use register::{mmio::*, register_bitfields, register_structs};


// GPIO registers.
//
// Descriptions taken from
// - https://github.com/raspberrypi/documentation/files/1888662/BCM2837-ARM-Peripherals.-.Revised.-.V2-1.pdf
// - https://datasheets.raspberrypi.org/bcm2711/bcm2711-peripherals.pdf
register_bitfields! {
    u32,

    /// GPIO Function Select 1
    GPFSEL1 [
        /// Pin 15
        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100  // PL011 UART RX

        ],

        /// Pin 14
        FSEL14 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100  // PL011 UART TX
        ]
    ],

    /// GPIO Pull-up/down Register
    ///
    /// BCM2837 only.
    GPPUD [
        /// Controls the actuation of the internal pull-up/down control line to ALL the GPIO pins.
        PUD OFFSET(0) NUMBITS(2) [
            Off = 0b00,
            PullDown = 0b01,
            PullUp = 0b10
        ]
    ],

    /// GPIO Pull-up/down Clock Register 0
    ///
    /// BCM2837 only.
    GPPUDCLK0 [
        /// Pin 15
        PUDCLK15 OFFSET(15) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ],

        /// Pin 14
        PUDCLK14 OFFSET(14) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ]
    ],

    /// GPIO Pull-up / Pull-down Register 0
    ///
    /// BCM2711 only.
    GPIO_PUP_PDN_CNTRL_REG0 [
        /// Pin 15
        GPIO_PUP_PDN_CNTRL15 OFFSET(30) NUMBITS(2) [
            NoResistor = 0b00,
            PullUp = 0b01
        ],

        /// Pin 14
        GPIO_PUP_PDN_CNTRL14 OFFSET(28) NUMBITS(2) [
            NoResistor = 0b00,
            PullUp = 0b01
        ]
    ]
}

/// Possibles values for the FSEL fields in GPFSEL register
enum GpioMode {
    Input = 0b000,
    Output = 0b001,
    AltFunc0 = 0b100,
    AltFunc1 = 0b101,
    AltFunc2 = 0b110,
    AltFunc3 = 0b111,
    AltFunc4 = 0b011,
    AltFunc5 = 0b010,
}

/// Possibles values for the GPIO_PUP_PDN_CNTRL fields in GPIO_PUP_PDN_CNTRL_REG register
#[allow(non_camel_case_types)]
enum GpioResistor {
    NoResistor = 0b00,
    PullUp = 0b01,
    PullDown = 0b10,
}

register_structs! {
    #[allow(non_snake_case)]
    RegisterBlock {
        (0x00 => GPFSEL: [ReadWrite<u32>; 6]),
        (0x18 => _reserved0),
        (0x1C => GPSET: [WriteOnly<u32>; 2]),
        (0x24 => _reserved1),
        (0x28 => GPCLR: [WriteOnly<u32>; 2]),
        (0x30 => _reserved2),
        (0x34 => GPLEV: [ReadOnly<u32>; 2]),
        (0x3C => _reserved3),
        (0x40 => GPEDS: [WriteOnly<u32>; 2]),
        (0x48 => _reserved4),
        (0x4C => GPREN: [ReadWrite<u32>; 2]),
        (0x54 => _reserved5),
        (0x58 => GPFEN: [ReadWrite<u32>; 2]),
        (0x60 => _reserved6),
        (0x64 => GPHEN: [ReadWrite<u32>; 2]),
        (0x6C => _reserved7),
        (0x70 => GPLEN: [ReadWrite<u32>; 2]),
        (0x78 => _reserved8),
        (0x7C => GPAREN: [ReadWrite<u32>; 2]),
        (0x84 => _reserved9),
        (0x88 => GPAFEN: [ReadWrite<u32>; 2]),
        (0x90 => _reserved10),
        (0xE4 => GPIO_PUP_PDN_CNTRL_REG: [ReadWrite<u32>; 2]),
        (0xF4 => @END),
    }
}

pub struct GPIO {
    base_addr: usize
}

impl core::ops::Deref for GPIO {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl GPIO {
    /// Create an instance.
    ///
    /// # Safety
    ///
    /// - The user must ensure to provide a correct MMIO start address.
    pub unsafe fn new(base_addr: usize) -> Self {
        Self {
            base_addr,
        }
    }

    fn ptr(&self) -> *const RegisterBlock {
        self.base_addr as *const _
    }

    pub fn set_mode(&mut self, pin: u8, mode: GpioMode) {
        assert!(pin <= 57);
        let reg = match pin {
            0..9 => self.GPFSEL[0],
        }
    }

    /// Disable pull-up/down on pins 14 and 15.
    #[cfg(feature = "bsp_rpi3")]
    fn disable_pud_14_15_bcm2837(&mut self) {
        use crate::cpu;

        // Make an educated guess for a good delay value (Sequence described in the BCM2837
        // peripherals PDF).
        //
        // - According to Wikipedia, the fastest Pi3 clocks around 1.4 GHz.
        // - The Linux 2837 GPIO driver waits 1 µs between the steps.
        //
        // So lets try to be on the safe side and default to 2000 cycles, which would equal 1 µs
        // would the CPU be clocked at 2 GHz.
        const DELAY: usize = 2000;

        self.registers.GPPUD.write(GPPUD::PUD::Off);
        cpu::spin_for_cycles(DELAY);

        self.registers
            .GPPUDCLK0
            .write(GPPUDCLK0::PUDCLK15::AssertClock + GPPUDCLK0::PUDCLK14::AssertClock);
        cpu::spin_for_cycles(DELAY);

        self.registers.GPPUD.write(GPPUD::PUD::Off);
        self.registers.GPPUDCLK0.set(0);
    }

    /// Disable pull-up/down on pins 14 and 15.
    fn disable_pud_14_15_bcm2711(&mut self) {
        self.registers.GPIO_PUP_PDN_CNTRL_REG0.write(
            GPIO_PUP_PDN_CNTRL_REG0::GPIO_PUP_PDN_CNTRL15::PullUp
                + GPIO_PUP_PDN_CNTRL_REG0::GPIO_PUP_PDN_CNTRL14::PullUp,
        );
    }

    /// Map PL011 UART as standard output.
    ///
    /// TX to pin 14
    /// RX to pin 15
    pub fn map_pl011_uart(&mut self) {
        // Select the UART on pins 14 and 15.
        self.registers
            .GPFSEL1
            .modify(GPFSEL1::FSEL15::AltFunc0 + GPFSEL1::FSEL14::AltFunc0);

        self.disable_pud_14_15_bcm2711();
    }
}
