use core::marker::PhantomData;

#[cfg(feature = "rpi4")]
mod rpi4 {
    use crate::{gpio, uart, gicv2};
    use core::{marker::PhantomData, ops::Deref};

    pub mod mmio {
        pub const GPIO_OFFSET: usize = 0x0020_0000;
        pub const UART0_OFFSET: usize = 0x0020_1000;
        pub const UART2_OFFSET: usize = 0x0020_1400;
        pub const UART3_OFFSET: usize = 0x0020_1600;
        pub const UART4_OFFSET: usize = 0x0020_1800;
        pub const UART5_OFFSET: usize = 0x0020_1A00;

        pub const START: usize = 0xFE00_0000;
        pub const GPIO_START: usize = START + GPIO_OFFSET;
        pub const UART0_START: usize = START + UART0_OFFSET;
        pub const UART2_START: usize = START + UART2_OFFSET;
        pub const UART3_START: usize = START + UART3_OFFSET;
        pub const UART4_START: usize = START + UART4_OFFSET;
        pub const UART5_START: usize = START + UART5_OFFSET;
        pub const GICD_START: usize = 0xFF84_1000;
        pub const GICC_START: usize = 0xFF84_2000;
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

    pub struct Uart0 {
        pub(crate) _marker: PhantomData<*const ()>,
    }

    unsafe impl Send for Uart0 {}

    impl Uart0 {
        #[inline(always)]
        pub const fn ptr() -> *const uart::RegisterBlock {
            mmio::UART0_START as *const _
        }
    }

    impl Deref for Uart0 {
        type Target = uart::RegisterBlock;

        #[inline(always)]
        fn deref(&self) -> &Self::Target {
            unsafe { &*Uart0::ptr() }
        }
    }

    pub struct Uart2 {
        pub(crate) _marker: PhantomData<*const ()>,
    }

    unsafe impl Send for Uart2 {}

    impl Uart2 {
        #[inline(always)]
        pub const fn ptr() -> *const uart::RegisterBlock {
            mmio::UART2_START as *const _
        }
    }

    impl Deref for Uart2 {
        type Target = uart::RegisterBlock;

        #[inline(always)]
        fn deref(&self) -> &Self::Target {
            unsafe { &*Uart2::ptr() }
        }
    }

    pub struct Uart3 {
        pub(crate) _marker: PhantomData<*const ()>,
    }

    unsafe impl Send for Uart3 {}

    impl Uart3 {
        #[inline(always)]
        pub const fn ptr() -> *const uart::RegisterBlock {
            mmio::UART3_START as *const _
        }
    }

    impl Deref for Uart3 {
        type Target = uart::RegisterBlock;

        #[inline(always)]
        fn deref(&self) -> &Self::Target {
            unsafe { &*Uart3::ptr() }
        }
    }

    pub struct Uart4 {
        pub(crate) _marker: PhantomData<*const ()>,
    }

    unsafe impl Send for Uart4 {}

    impl Uart4 {
        #[inline(always)]
        pub const fn ptr() -> *const uart::RegisterBlock {
            mmio::UART4_START as *const _
        }
    }

    impl Deref for Uart4 {
        type Target = uart::RegisterBlock;

        #[inline(always)]
        fn deref(&self) -> &Self::Target {
            unsafe { &*Uart4::ptr() }
        }
    }

    pub struct Uart5 {
        pub(crate) _marker: PhantomData<*const ()>,
    }

    unsafe impl Send for Uart5 {}

    impl Uart5 {
        #[inline(always)]
        pub const fn ptr() -> *const uart::RegisterBlock {
            mmio::UART5_START as *const _
        }
    }

    impl Deref for Uart5 {
        type Target = uart::RegisterBlock;

        #[inline(always)]
        fn deref(&self) -> &Self::Target {
            unsafe { &*Uart5::ptr() }
        }
    }

    pub struct Gicc {
        pub(crate) _marker: PhantomData<*const ()>,
    }

    unsafe impl Send for Gicc {}

    impl Gicc {
        #[inline(always)]
        pub const fn ptr() -> *const gicv2::gicc::RegisterBlock {
            mmio::GICC_START as *const _
        }
    }

    impl Deref for Gicc {
        type Target = gicv2::gicc::RegisterBlock;

        #[inline(always)]
        fn deref(&self) -> &Self::Target {
            unsafe { &*Gicc::ptr() }
        }
    }

    pub struct GicdShared {
        pub(crate) _marker: PhantomData<*const ()>,
    }

    unsafe impl Send for GicdShared {}

    impl GicdShared {
        #[inline(always)]
        pub const fn ptr() -> *const gicv2::gicd::SharedRegisterBlock {
            mmio::GICD_START as *const _
        }
    }

    impl Deref for GicdShared {
        type Target = gicv2::gicd::SharedRegisterBlock;

        #[inline(always)]
        fn deref(&self) -> &Self::Target {
            unsafe { &*GicdShared::ptr() }
        }
    }

    pub struct GicdBanked {
        pub(crate) _marker: PhantomData<*const ()>,
    }

    unsafe impl Send for GicdBanked {}

    impl GicdBanked {
        #[inline(always)]
        pub const fn ptr() -> *const gicv2::gicd::BankedRegisterBlock {
            mmio::GICD_START as *const _
        }
    }

    impl Deref for GicdBanked {
        type Target = gicv2::gicd::BankedRegisterBlock;

        #[inline(always)]
        fn deref(&self) -> &Self::Target {
            unsafe { &*GicdBanked::ptr() }
        }
    }
}

#[cfg(feature = "rpi4")]
pub use rpi4::*;

pub struct Peripherals {
    #[cfg(feature = "rpi4")]
    pub gpio: Gpio,
    #[cfg(feature = "rpi4")]
    pub uart0: Uart0,
    #[cfg(feature = "rpi4")]
    pub uart2: Uart2,
    #[cfg(feature = "rpi4")]
    pub uart3: Uart3,
    #[cfg(feature = "rpi4")]
    pub uart4: Uart4,
    #[cfg(feature = "rpi4")]
    pub uart5: Uart5,
    #[cfg(feature = "rpi4")]
    pub gicc: Gicc,
    #[cfg(feature = "rpi4")]
    pub gicd_shared: GicdShared,
    #[cfg(feature = "rpi4")]
    pub gicd_banked: GicdBanked,
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
            #[cfg(feature = "rpi4")]
            uart0: Uart0 {
                _marker: PhantomData,
            },
            #[cfg(feature = "rpi4")]
            uart2: Uart2 {
                _marker: PhantomData,
            },
            #[cfg(feature = "rpi4")]
            uart3: Uart3 {
                _marker: PhantomData,
            },
            #[cfg(feature = "rpi4")]
            uart4: Uart4 {
                _marker: PhantomData,
            },
            #[cfg(feature = "rpi4")]
            uart5: Uart5 {
                _marker: PhantomData,
            },
            #[cfg(feature = "rpi4")]
            gicc: Gicc {
                _marker: PhantomData,
            },
            #[cfg(feature = "rpi4")]
            gicd_shared: GicdShared {
                _marker: PhantomData,
            },
            #[cfg(feature = "rpi4")]
            gicd_banked: GicdBanked {
                _marker: PhantomData,
            },
        }
    }
}
