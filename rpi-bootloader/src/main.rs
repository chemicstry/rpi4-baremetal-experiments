#![no_main]
#![no_std]
#![feature(global_asm)]

use null_lock::NullLock;
use rpi_hal::gpio::GpioParts;
use rpi_hal::prelude::*;
use rpi_hal::serial::Serial;
use xmodem::Xmodem;

use core::fmt::Write;

extern crate cortex_a_rt;
extern crate panic_halt;

mod boot;
mod null_lock;

const UART_BAUD: Bps = Bps(921200);
const FW_LOAD_ADDR: usize = 0x80000;
const FW_MAX_SIZE: usize = 32 * 1024 * 1024; // 32MB

struct MemWriter {
    addr: usize,
    addr_end: usize,
}

impl MemWriter {
    unsafe fn new(start_addr: usize, size: usize) -> MemWriter {
        MemWriter {
            addr: start_addr,
            addr_end: start_addr + size,
        }
    }
}

impl xmodem::io::Write for MemWriter {
    fn write_all(&mut self, buf: &[u8]) -> xmodem::io::Result<()> {
        if buf.len() > self.addr_end - self.addr {
            return Err(xmodem::io::Error::Other("Not enough space"));
        }

        let dst = unsafe { core::slice::from_raw_parts_mut(self.addr as *mut u8, buf.len()) };
        dst.copy_from_slice(buf);
        self.addr += buf.len();

        Ok(())
    }
}

#[no_mangle]
fn main() -> ! {
    // We make sure that we call steal only once
    let dp = unsafe { rpi_hal::rpi::Peripherals::steal() };

    let gpio = rpi_hal::gpio::Gpio::<NullLock>::new(dp.gpio);
    let pins = GpioParts::split(&gpio);

    // Setup UART pins
    let tx = pins.gpio14.into_alt_func0().into_floating();
    let rx = pins.gpio15.into_alt_func0().into_floating();

    let uart_config = rpi_hal::serial::config::Config::default().baudrate(UART_BAUD);
    let mut uart = Serial::new(dp.uart0, (tx, rx), uart_config, 48.mhz().into());

    writeln!(uart, "RPi Bootloader").ok();
    writeln!(uart, "Trying XMODEM transfer...").ok();

    let mut xmodem = Xmodem::new();
    let mut mem_writer = unsafe { MemWriter::new(FW_LOAD_ADDR, FW_MAX_SIZE) };

    loop {
        match xmodem.recv(&mut uart, &mut mem_writer, xmodem::Checksum::Standard) {
            Ok(_) => {
                writeln!(uart, "Firmware loaded").ok();
                break;
            }
            Err(e) => {
                writeln!(uart, "Error loading firmware: {:?}", e).ok();
                continue;
            }
        }
    }

    writeln!(uart, "Jumping to firmware at {}", FW_LOAD_ADDR).ok();

    let jump: fn() -> ! = unsafe { core::mem::transmute(FW_LOAD_ADDR) };
    jump();
}
