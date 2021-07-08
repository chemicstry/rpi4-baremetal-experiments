#![no_std]

pub mod gicv2;
pub mod gpio;
pub mod serial;
pub mod time;

#[cfg(feature = "panic_uart")]
pub mod panic_uart;

pub use rpi_pac as rpi;

pub mod prelude {
    pub use crate::time::*;
}
