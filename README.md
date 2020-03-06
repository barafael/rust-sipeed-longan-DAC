# rust-sipeed-longan-DAC
A minimal example of using the DAC on the RISC-V Sipeed Longan Nano via Rust.

DISCLAIMER: I am fully aware that programming on bare registers like this is not the idiomatic way to program microcontrollers in Rust. A HAL or at least PAC crate could have been used, but I wanted to try my hand at this sort of low level programming in Rust.

These functions and defines have been strongly inspired by the [GD32VF103_Firmware_Library](https://github.com/riscv-mcu/GD32VF103_Firmware_Library).

Note: For a working version of this using the PAC, check out the rust-pac branch!
