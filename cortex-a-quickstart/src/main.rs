#![no_main]
#![no_std]
#![feature(global_asm)]

use null_lock::NullLock;
use rpi_hal::gpio::GpioParts;
use rpi_hal::prelude::*;
use rpi_hal::serial::Serial;

extern crate cortex_a_rt;

use core::fmt::Write;
use core::panic::PanicInfo;
mod null_lock;

#[no_mangle]
fn main() -> ! {
    // We make sure that we call steal only once
    let dp = unsafe { rpi_hal::rpi::Peripherals::steal() };

    let gpio = rpi_hal::gpio::Gpio::<NullLock>::new(dp.gpio);
    let pins = GpioParts::split(&gpio);

    // Setup UART pins
    let tx = pins.gpio14.into_alt_func0().into_floating();
    let rx = pins.gpio15.into_alt_func0().into_floating();

    let uart_config = rpi_hal::serial::config::Config::default().baudrate(921200.bps());
    let mut uart = Serial::new(dp.uart0, (tx, rx), uart_config, 48.mhz().into());

    for byte in b"Hello, world!" {
        nb::block!(uart.write(*byte)).unwrap();
    }

    let big_addr: u64 = 9 * 1024 * 1024 * 1024;
    unsafe { core::ptr::read_volatile(big_addr as *mut u64) };

    panic!("test");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let dp = unsafe { rpi_hal::rpi::Peripherals::steal() };

    let gpio = rpi_hal::gpio::Gpio::<NullLock>::new(dp.gpio);
    let pins = GpioParts::split(&gpio);

    // Setup UART pins
    let tx = pins.gpio14.into_alt_func0().into_floating();
    let rx = pins.gpio15.into_alt_func0().into_floating();

    let uart_config = rpi_hal::serial::config::Config::default().baudrate(921200.bps());
    let mut uart = Serial::new(dp.uart0, (tx, rx), uart_config, 48.mhz().into());

    writeln!(uart, "{}", info).ok();

    loop {
        cortex_a::asm::wfe()
    }
}
