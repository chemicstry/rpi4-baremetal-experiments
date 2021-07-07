# RPi-RTIC

This is a monorepo for [RTIC](http://rtic.rs) implementation on Raspberry Pi 4.

Repository is set up as a cargo workspace containing multiple crates to simplify development. They are not yet released on crates.io

## Crates

Summary:
- [rpi-pac](rpi-pac/) (Raspberry Peripheral Access Crate)
  - Contains register definitions of RPi4 peripherals.
- [rpi-hal](rpi-hal/) (Raspberry Hardware Abstraction Layer)
  - A crate that wraps registers from `rpi-pac` and provides high-level functions for working with peripherals.
  - Most peripherals implement [embedded-hal](https://github.com/rust-embedded/embedded-hal) traits.
- [cortex-a-rt](cortex-a-rt/) (Cortex-A Runtime)
  - Contains linker script and low-level initialization code to load into rust code.
- [cortex-a-quickstart](cortex-a-quickstart/)
  - A example code, which compiles to a working binary and demonstrates the usage of all the crates.
- [rpi-bootloader](rpi-bootloader/)
  - A bootloader for loading firmware binaries over UART. This saves time from having to flash SD card each time.

## References

Relevant documentation is stored under [docs](docs/).

A lot of the inspiration (or code snippets) came from other projects:
- https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials
  - A really good tutorial on working with RPi from Rust.
  - Unfortunatelly, it is implemented as a monolithic kernel and code is hard to reuse.
- https://github.com/RusPiRo
  - A large ecosystem of RPi3 crates.
  - Does not integrate well with the rest of rust embedded ecosystem and does not support RPi4.
- https://github.com/stm32-rs
  - Rust crates for STM32 microcontrollers.
  - Used as inspiration for implementing `rpi-pac` and `rpi-hal` crates.
