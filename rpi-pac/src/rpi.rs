use core::marker::PhantomData;

#[cfg(feature = "rpi4")]
mod rpi4 {
    use crate::gpio;
    use core::{marker::PhantomData, ops::Deref};

    pub mod mmio {
        pub const GPIO_OFFSET: usize = 0x0020_0000;
        pub const UART_OFFSET: usize = 0x0020_1000;

        pub const START: usize = 0xFE00_0000;
        pub const GPIO_START: usize = START + GPIO_OFFSET;
        pub const PL011_UART_START: usize = START + UART_OFFSET;
    }

    pub struct Gpio {
        pub(crate) _marker: PhantomData<*const ()>,
    }

    unsafe impl Send for Gpio {}

    impl Gpio {
        #[inline(always)]
        pub const fn ptr() -> *const gpio::RegisterBlock {
            mmio::GPIO_START as *const _
        }
    }

    impl Deref for Gpio {
        type Target = gpio::RegisterBlock;

        #[inline(always)]
        fn deref(&self) -> &Self::Target {
            unsafe { &*Gpio::ptr() }
        }
    }
}

#[cfg(feature = "rpi4")]
pub use rpi4::*;

pub struct Peripherals {
    #[cfg(feature = "rpi4")]
    pub gpio: Gpio,
}

impl Peripherals {
    /// Returns all device peripherals.
    ///
    /// # Safety
    ///
    /// Must be called only once to prevent aliasing.
    pub unsafe fn steal() -> Peripherals {
        Peripherals {
            #[cfg(feature = "rpi4")]
            gpio: Gpio {
                _marker: PhantomData,
            },
        }
    }
}
