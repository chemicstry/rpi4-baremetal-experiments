#![no_main]
#![no_std]
#![feature(global_asm)]

// use rpi_hal::{gpio::Gpio, uart::PL011Uart};
// use cortex_a::asm;
// use embedded_hal::prelude::*;

extern crate cortex_a_rt;
extern crate panic_halt;

mod boot;

#[no_mangle]
fn main() -> ! {
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
