#![no_std]

pub mod gpio;
pub mod serial;
pub mod time;

pub use rpi_pac as rpi;

pub mod prelude {
    pub use crate::time::*;
}
