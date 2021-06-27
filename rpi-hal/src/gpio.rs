use core::{convert::Infallible, marker::PhantomData};

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

const NUM_GPIO_PINS: u8 = 58;

/// Generic implementation of the GPIO peripheral.
/// A mutex implementation must be provided to ensure register consistency in multi-core environment.
/// If GPIO is to be used by single core only, a NullLock can be used instead.
pub struct Gpio<LOCK: RawMutex> {
    // In multicore systems we have to lock registers for read-modify-write operations
    gpio: Mutex<LOCK, pac::rpi::Gpio>,
}

impl<LOCK: RawMutex> Gpio<LOCK> {
    pub fn new(gpio: pac::rpi::Gpio) -> Self {
        Self {
            gpio: Mutex::new(gpio),
        }
    }
}

impl<LOCK: RawMutex> GpioExt for Gpio<LOCK> {
    fn set_mode(&self, pin: u8, mode: GpioMode) {
        assert!(pin < NUM_GPIO_PINS);
        let reg = &self.gpio.lock().gpfsel[(pin / 10) as usize];
        let field = Field::<u32, ()>::new(0b111, ((pin % 10) * 3) as usize);
        reg.modify(field.val(mode as u32));
    }

    fn set_resistor(&self, pin: u8, res: GpioResistor) {
        assert!(pin < NUM_GPIO_PINS);
        let reg = &self.gpio.lock().gpio_pup_pdn_cntrl_reg[(pin / 16) as usize];
        let field = Field::<u32, ()>::new(0b11, ((pin % 16) * 2) as usize);
        reg.modify(field.val(res as u32));
    }

    fn set_high(&self, pin: u8) {
        assert!(pin < NUM_GPIO_PINS);
        // No need to lock because register supports atomic writes
        unsafe {
            let gpio = &*self.gpio.data_ptr();
            let reg = &gpio.gpset[(pin / 32) as usize];
            reg.set(1 << (pin % 32));
        }
    }

    fn set_low(&self, pin: u8) {
        assert!(pin < NUM_GPIO_PINS);
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
        assert!(pin < NUM_GPIO_PINS);
        // No need to lock because register is read only
        unsafe {
            let gpio = &*self.gpio.data_ptr();
            let reg = &gpio.gplev[(pin / 32) as usize];
            reg.get() & (1 << (pin % 32)) == 0
        }
    }
}

macro_rules! gpio_parts {
    ($GPIOPARTS:ident, [$($GPIOi:ident: $i:expr,)+]) => {
        pub struct $GPIOPARTS<'a, GPIO: GpioExt + 'a> {
            $(
                pub $GPIOi: Pin<'a, GPIO, Input, Floating, $i>,
            )+
        }

        impl<'a, GPIO: GpioExt + 'a> $GPIOPARTS<'a, GPIO> {
            pub fn split(gpio: &'a GPIO) -> Self {
                Self {
                    $(
                        $GPIOi: Pin::new(gpio),
                    )+
                }
            }
        }
    }
}

// Declarative macros do not support automatic repeat
gpio_parts!(GpioParts, [
    gpio0: 0,
    gpio1: 1,
    gpio2: 2,
    gpio3: 3,
    gpio4: 4,
    gpio5: 5,
    gpio6: 6,
    gpio7: 7,
    gpio8: 8,
    gpio9: 9,
    gpio10: 10,
    gpio11: 11,
    gpio12: 12,
    gpio13: 13,
    gpio14: 14,
    gpio15: 15,
    gpio16: 16,
    gpio17: 17,
    gpio18: 18,
    gpio19: 19,
    gpio20: 20,
    gpio21: 21,
    gpio22: 22,
    gpio23: 23,
    gpio24: 24,
    gpio25: 25,
    gpio26: 26,
    gpio27: 27,
    gpio28: 28,
    gpio29: 29,
    gpio30: 30,
    gpio31: 31,
    gpio32: 32,
    gpio33: 33,
    gpio34: 34,
    gpio35: 35,
    gpio36: 36,
    gpio37: 37,
    gpio38: 38,
    gpio39: 39,
    gpio40: 40,
    gpio41: 41,
    gpio42: 42,
    gpio43: 43,
    gpio44: 44,
    gpio45: 45,
    gpio46: 46,
    gpio47: 47,
    gpio48: 48,
    gpio49: 49,
    gpio50: 50,
    gpio51: 51,
    gpio52: 52,
    gpio53: 53,
    gpio54: 54,
    gpio55: 55,
    gpio56: 56,
    gpio57: 57,
]);

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

    pub fn into_floating(self) -> Pin<'a, GPIO, MODE, Floating, INDEX> {
        self.gpio.set_resistor(INDEX, GpioResistor::Floating);
        Pin {
            gpio: self.gpio,
            _marker: Default::default(),
        }
    }

    pub fn into_pull_up(self) -> Pin<'a, GPIO, MODE, PullUp, INDEX> {
        self.gpio.set_resistor(INDEX, GpioResistor::PullUp);
        Pin {
            gpio: self.gpio,
            _marker: Default::default(),
        }
    }

    pub fn into_pull_down(self) -> Pin<'a, GPIO, MODE, PullDown, INDEX> {
        self.gpio.set_resistor(INDEX, GpioResistor::PullDown);
        Pin {
            gpio: self.gpio,
            _marker: Default::default(),
        }
    }
}

impl<'a, GPIO: GpioExt, PU, const INDEX: u8> embedded_hal::digital::v2::OutputPin
    for Pin<'a, GPIO, Output, PU, INDEX>
{
    type Error = Infallible;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.gpio.set_low(INDEX);
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.gpio.set_high(INDEX);
        Ok(())
    }
}

impl<'a, GPIO: GpioExt, PU, const INDEX: u8> embedded_hal::digital::v2::StatefulOutputPin
    for Pin<'a, GPIO, Output, PU, INDEX>
{
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self.gpio.is_high(INDEX))
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(self.gpio.is_low(INDEX))
    }
}

impl<'a, GPIO: GpioExt, PU, const INDEX: u8> embedded_hal::digital::v2::ToggleableOutputPin
    for Pin<'a, GPIO, Output, PU, INDEX>
{
    type Error = Infallible;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        if self.gpio.is_high(INDEX) {
            self.gpio.set_low(INDEX)
        } else {
            self.gpio.set_high(INDEX)
        }

        Ok(())
    }
}

impl<'a, GPIO: GpioExt, PU, const INDEX: u8> embedded_hal::digital::v2::InputPin
    for Pin<'a, GPIO, Output, PU, INDEX>
{
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.gpio.is_high(INDEX))
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.gpio.is_low(INDEX))
    }
}
