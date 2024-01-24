# RPi4 baremetal experiments

This repository contains baremetal experiments on Raspberry Pi 4, which was part of my Master's thesis [Real-Time Interrupt-driven Concurrency (RTIC) for Modern Application Processors](https://essay.utwente.nl/89253/) at University of Twente.

I intended to develop and release a set of universal crates (similar to other MCU PAC and HAL crates), which could be used for writing baremetal apps on RPi4, but the project scope turned out to be much larger than I expected. GIC, MMU, multiple cores and other CPU features add a lot of complexity and make most of the code unsafe, where a large framework is needed to wrap everything under a memory safe API. I'm no longer interested in developing this further, however, someone might find this repository useful in their endeavors. Feel free to fork it or ask questions in the issues.

## Crates

Repository is set up as a cargo workspace containing multiple crates to simplify development.

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
