use register::{mmio::*, register_bitfields, register_structs};

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
enum GpioResistor {
    Floating = 0b00,
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
    /// - The user must ensure to provide a correct MMIO base address.
    /// - There should be no aliases to this address.
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

pub struct Output;
pub struct Input;

pub struct Pin<MODE, PUD> {
    base_addr: usize,
    pin: u8,
}
