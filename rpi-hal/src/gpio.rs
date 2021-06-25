use core::marker::PhantomData;

use lock_api::{Mutex, RawMutex};
use register::Field;
use rpi_pac as pac;

/// Possibles values for the FSEL fields in GPFSEL register
pub enum GpioMode {
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
pub enum GpioResistor {
    Floating = 0b00,
    PullUp = 0b01,
    PullDown = 0b10,
}

pub trait GpioExt {
    fn set_mode(&self, pin: u8, mode: GpioMode);
    fn set_resistor(&self, pin: u8, res: GpioResistor);
    fn set_high(&self, pin: u8);
    fn set_low(&self, pin: u8);
    fn is_high(&self, pin: u8) -> bool;
    fn is_low(&self, pin: u8) -> bool;
}

const NUM_GPIO_PINS: u8 = 57;

pub struct Gpio<LOCK: RawMutex> {
    gpio: Mutex<LOCK, pac::rpi::Gpio>,
}

pub struct GpioParts<'a, GPIO: GpioExt + 'a> {
    pub gpio0: Pin<'a, GPIO, Input, Floating, 0>,
}

impl<LOCK: RawMutex> Gpio<LOCK> {
    pub fn new(gpio: pac::rpi::Gpio) -> Self {
        Self { 
            gpio: Mutex::new(gpio),
        }
    }

    pub fn split<'a>(&'a self) -> GpioParts<'a, Gpio<LOCK>> {
        GpioParts {
            gpio0: Pin::new(self),
        }
    }
}

impl<LOCK: RawMutex> GpioExt for Gpio<LOCK> {
    fn set_mode(&self, pin: u8, mode: GpioMode) {
        assert!(pin <= NUM_GPIO_PINS);
        let reg = &self.gpio.lock().gpfsel[(pin / 10) as usize];
        let field = Field::<u32, ()>::new(0b111, ((pin % 10) * 3) as usize);
        reg.modify(field.val(mode as u32));
    }

    fn set_resistor(&self, pin: u8, res: GpioResistor) {
        assert!(pin <= NUM_GPIO_PINS);
        let reg = &self.gpio.lock().gpio_pup_pdn_cntrl_reg[(pin / 16) as usize];
        let field = Field::<u32, ()>::new(0b11, ((pin % 16) * 2) as usize);
        reg.modify(field.val(res as u32));
    }

    fn set_high(&self, pin: u8) {
        assert!(pin <= NUM_GPIO_PINS);
        // No need to lock because register supports atomic writes
        unsafe {
            let gpio = &*self.gpio.data_ptr();
            let reg = &gpio.gpset[(pin / 32) as usize];
            reg.set(1 << (pin % 32));
        }
    }

    fn set_low(&self, pin: u8) {
        assert!(pin <= NUM_GPIO_PINS);
        // No need to lock because register supports atomic writes
        unsafe {
            let gpio = &*self.gpio.data_ptr();
            let reg = &gpio.gpclr[(pin / 32) as usize];
            reg.set(1 << (pin % 32));
        }
    }

    fn is_high(&self, pin: u8) -> bool {
        !self.is_low(pin)
    }

    fn is_low(&self, pin: u8) -> bool {
        assert!(pin <= NUM_GPIO_PINS);
        // No need to lock because register is read only
        unsafe {
            let gpio = &*self.gpio.data_ptr();
            let reg = &gpio.gplev[(pin / 32) as usize];
            reg.get() & (1 << (pin % 32)) == 0
        }
    }
}

pub mod typestate {
    pub struct Input;
    pub struct Output;
    pub struct AltFunc0;
    pub struct AltFunc1;
    pub struct AltFunc2;
    pub struct AltFunc3;
    pub struct AltFunc4;
    pub struct AltFunc5;
    pub struct Floating;
    pub struct PullDown;
    pub struct PullUp;
}

use typestate::*;

pub struct Pin<'a, GPIO: GpioExt + 'a, MODE, PU, const INDEX: u8> {
    // In multicore systems we have to lock registers that do read-modify-write
    gpio: &'a GPIO,
    _marker: PhantomData<(MODE, PU)>,
}

impl<'a, GPIO: GpioExt, MODE, PU, const INDEX: u8> Pin<'a, GPIO, MODE, PU, INDEX> {
    // Private constructor to ensure there only exists one of each pin.
    fn new(gpio: &'a GPIO) -> Self {
        Self {
            gpio,
            _marker: Default::default(),
        }
    }

    pub fn into_input(self) -> Pin<'a, GPIO, Input, PU, INDEX> {
        self.gpio.set_mode(INDEX, GpioMode::Input);
        Pin {
            gpio: self.gpio,
            _marker: Default::default(),
        }
    }

    pub fn into_output(self) -> Pin<'a, GPIO, Output, PU, INDEX> {
        self.gpio.set_mode(INDEX, GpioMode::Output);
        Pin {
            gpio: self.gpio,
            _marker: Default::default(),
        }
    }

    pub fn into_alt_func0(self) -> Pin<'a, GPIO, AltFunc0, PU, INDEX> {
        self.gpio.set_mode(INDEX, GpioMode::AltFunc0);
        Pin {
            gpio: self.gpio,
            _marker: Default::default(),
        }
    }

    pub fn into_alt_func1(self) -> Pin<'a, GPIO, AltFunc1, PU, INDEX> {
        self.gpio.set_mode(INDEX, GpioMode::AltFunc1);
        Pin {
            gpio: self.gpio,
            _marker: Default::default(),
        }
    }

    pub fn into_alt_func2(self) -> Pin<'a, GPIO, AltFunc2, PU, INDEX> {
        self.gpio.set_mode(INDEX, GpioMode::AltFunc2);
        Pin {
            gpio: self.gpio,
            _marker: Default::default(),
        }
    }

    pub fn into_alt_func3(self) -> Pin<'a, GPIO, AltFunc3, PU, INDEX> {
        self.gpio.set_mode(INDEX, GpioMode::AltFunc3);
        Pin {
            gpio: self.gpio,
            _marker: Default::default(),
        }
    }

    pub fn into_alt_func4(self) -> Pin<'a, GPIO, AltFunc4, PU, INDEX> {
        self.gpio.set_mode(INDEX, GpioMode::AltFunc4);
        Pin {
            gpio: self.gpio,
            _marker: Default::default(),
        }
    }

    pub fn into_alt_func5(self) -> Pin<'a, GPIO, AltFunc5, PU, INDEX> {
        self.gpio.set_mode(INDEX, GpioMode::AltFunc5);
        Pin {
            gpio: self.gpio,
            _marker: Default::default(),
        }
    }
}
