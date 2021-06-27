#![no_main]
#![no_std]
#![feature(global_asm)]

use null_lock::NullLock;
use rpi_hal::gpio::GpioParts;

// use rpi_hal::{gpio::Gpio, uart::PL011Uart};
// use cortex_a::asm;
// use embedded_hal::prelude::*;

extern crate cortex_a_rt;
extern crate panic_halt;

mod boot;
mod null_lock;

#[no_mangle]
fn main() -> ! {
    // We make sure that we call steal only once
    let dp = unsafe { rpi_hal::rpi::Peripherals::steal() };

    let gpio = rpi_hal::gpio::Gpio::<NullLock>::new(dp.gpio);
    let pins = GpioParts::split(&gpio);

    // Setup UART pins
    pins.gpio14.into_alt_func0().into_floating();
    pins.gpio15.into_alt_func0().into_floating();

    //println!("Hello, world!");
    // let mut uart = unsafe {
    //     let mut gpio = GPIO::new(memory::mmio::GPIO_START);
    //     let mut uart = PL011Uart::new(memory::mmio::PL011_UART_START);
    //     uart.init();
    //     gpio.map_pl011_uart();
    //     uart
    // };

    // for byte in b"Hello, world!" {
    //     // NOTE `block!` blocks until `serial.write()` completes and returns
    //     // `Result<(), Error>`
    //     uart.write(*byte);
    // }

    loop {}
}
