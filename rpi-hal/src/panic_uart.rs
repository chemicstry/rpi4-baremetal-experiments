use crate::{
    gpio::{Gpio, GpioParts},
    prelude::*,
    rpi::Peripherals,
    serial::{self, Serial},
};
use core::fmt::Write;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let dp = unsafe { Peripherals::steal() };

    let gpio = Gpio::<NullLock>::new(dp.gpio);
    let pins = GpioParts::split(&gpio);

    // Setup UART pins
    let tx = pins.gpio14.into_alt_func0().into_floating();
    let rx = pins.gpio15.into_alt_func0().into_floating();

    // Initialized by RPi bootloader
    let uart_freq = 48.mhz().into();

    let uart_config = serial::config::Config::default().baudrate(921200.bps());
    let mut uart = Serial::new(dp.uart0, (tx, rx), uart_config, uart_freq);

    writeln!(uart, "{}", info).ok();

    loop {
        cortex_a::asm::wfe()
    }
}

/// Lock that doesn't do anything and is not safe for multiple cores.
/// Use spin locks instead when MMU is configured to support atomics.
pub struct NullLock {}

unsafe impl lock_api::RawMutex for NullLock {
    const INIT: NullLock = NullLock {};

    // A spinlock guard can be sent to another thread and unlocked there
    type GuardMarker = lock_api::GuardNoSend;

    fn lock(&self) {}

    fn try_lock(&self) -> bool {
        true
    }

    unsafe fn unlock(&self) {}
}
