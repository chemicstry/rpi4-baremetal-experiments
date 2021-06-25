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
    fn set_mode(&mut self, pin: u8, mode: GpioMode);
    fn set_resistor(&mut self, pin: u8, res: GpioResistor);
    fn set_high(&mut self, pin: u8);
    fn set_low(&mut self, pin: u8);
    fn is_high(&mut self, pin: u8) -> bool;
    fn is_low(&mut self, pin: u8);
}

pub struct Gpio<LOCK: RawMutex> {
    lock: Mutex<LOCK, ()>,
    gpio: pac::rpi::Gpio,
}

impl<LOCK: RawMutex> Gpio<LOCK> {
    pub fn new(gpio: pac::rpi::Gpio) -> Self {
        Self { 
            lock: Mutex::<LOCK, ()>::new(()),
            gpio
        }
    }

    const NUM_PINS: u8 = 57;
}

impl<LOCK: RawMutex> GpioExt for Gpio<LOCK> {
    fn set_mode(&mut self, pin: u8, mode: GpioMode) {
        assert!(pin <= Gpio::NUM_PINS);
        let reg = &self.gpio.gpfsel[(pin / 10) as usize];
        let field = Field::<u32, ()>::new(0b111, ((pin % 10) * 3) as usize);
        // Locking because of read-modify-write
        let _lock = self.lock.lock();
        reg.modify(field.val(mode as u32));
    }

    fn set_resistor(&mut self, pin: u8, res: GpioResistor) {
        assert!(pin <= Gpio::NUM_PINS);
        let reg = &self.gpio.gpio_pup_pdn_cntrl_reg[(pin / 16) as usize];
        let field = Field::<u32, ()>::new(0b11, ((pin % 16) * 2) as usize);
        // Locking because of read-modify-write
        let _lock = self.lock.lock();
        reg.modify(field.val(res as u32));
    }

    fn set_high(&mut self, pin: u8) {
        assert!(pin <= Gpio::NUM_PINS);
        let reg = &self.gpio.gpset[(pin / 32) as usize];
        // No need to lock because register supports atomic writes
        reg.set(1 << (pin % 32));
    }

    fn set_low(&mut self, pin: u8) {
        assert!(pin <= Gpio::NUM_PINS);
        let reg = &self.gpio.gpclr[(pin / 32) as usize];
        // No need to lock because register supports atomic writes
        reg.set(1 << (pin % 32));
    }

    fn is_high(&mut self, pin: u8) -> bool {
        !self.is_low(pin)
    }

    fn is_low(&mut self, pin: u8) -> bool {
        assert!(pin <= Gpio::NUM_PINS);
        let reg = &self.gpio.gplev[(pin / 32) as usize];
        // No need to lock because register is read only
        reg.get() & (1 << (pin % 32)) == 0
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
}

use typestate::*;

pub struct Pin<LOCK: RawMutex + 'static, MODE, PU, const INDEX: u8> {
    // In multicore systems we have to lock registers that do read-modify-write
    gpio: &'static Mutex<LOCK, Gpio>,
    _marker: PhantomData<(MODE, PU)>,
}

impl<LOCK: RawMutex, MODE, PU, const INDEX: u8> Pin<LOCK, MODE, PU, INDEX> {
    // Private constructor to ensure there only exists one of each pin.
    fn new(gpio: &'static Mutex<LOCK, Gpio>) -> Self {
        Self {
            gpio,
            _marker: Default::default(),
        }
    }

    pub fn into_input(self) -> Pin<LOCK, Input, PU, INDEX> {
        self.gpio.lock().set_mode(INDEX, GpioMode::Input);
        Pin {
            gpio: self.gpio,
            _marker: Default::default(),
        }
    }

    pub fn into_output(self) -> Pin<LOCK, Output, PU, INDEX> {
        unimplemented!()
    }
    pub fn into_alt_func0(self) -> Pin<LOCK, AltFunc0, PU, INDEX> {
        unimplemented!()
    }
    pub fn into_alt_func1(self) -> Pin<LOCK, AltFunc1, PU, INDEX> {
        unimplemented!()
    }
    pub fn into_alt_func2(self) -> Pin<LOCK, AltFunc2, PU, INDEX> {
        unimplemented!()
    }
    pub fn into_alt_func3(self) -> Pin<LOCK, AltFunc3, PU, INDEX> {
        unimplemented!()
    }
    pub fn into_alt_func4(self) -> Pin<LOCK, AltFunc4, PU, INDEX> {
        unimplemented!()
    }
    pub fn into_alt_func5(self) -> Pin<LOCK, AltFunc5, PU, INDEX> {
        unimplemented!()
    }
}

pub trait GpioExt {
    /// The parts to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins
    fn split(self) -> Self::Parts;
}
