#![no_main]
#![no_std]
#![feature(global_asm)]

use bcm_hal::{gpio::GPIO, uart::PL011Uart};
use cortex_a::asm;
use embedded_hal::prelude::*;

extern crate cortex_a_rt;
extern crate panic_halt;

mod boot;

pub mod memory {
    pub const GPIO_OFFSET: usize = 0x0020_0000;
    pub const UART_OFFSET: usize = 0x0020_1000;

    pub mod mmio {
        use super::*;

        pub const START: usize = 0xFE00_0000;
        pub const GPIO_START: usize = START + GPIO_OFFSET;
        pub const PL011_UART_START: usize = START + UART_OFFSET;
    }
}

#[no_mangle]
fn main() -> ! {
    //println!("Hello, world!");
    let mut uart = unsafe {
        let mut gpio = GPIO::new(memory::mmio::GPIO_START);
        let mut uart = PL011Uart::new(memory::mmio::PL011_UART_START);
        uart.init();
        gpio.map_pl011_uart();
        uart
    };

    for byte in b"Hello, world!" {
        // NOTE `block!` blocks until `serial.write()` completes and returns
        // `Result<(), Error>`
        uart.write(*byte);
    }

    loop {}
}
