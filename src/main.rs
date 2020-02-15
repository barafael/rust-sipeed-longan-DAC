#![feature(global_asm)]
#![no_main]
#![no_std]

use panic_abort;

mod dac;
mod gpio;
mod rcu;
mod register_helpers;

use dac::*;
use gpio::*;
use rcu::*;
pub use register_helpers::*;

use embedded_hal::blocking::delay::DelayMs;
use gd32vf103xx_hal as hal;
use gd32vf103xx_hal::delay::McycleDelay;
use gd32vf103xx_hal::gpio::GpioExt;
use gd32vf103xx_hal::prelude::*;
use gd32vf103xx_hal::rcu::RcuExt;
use hal::pac;
use longan_nano::led::{rgb, Led};
use longan_nano::sprintln;
use riscv_rt::entry;

fn rcu_config() {
    rcu_periph_clock_enable(RCU_GPIOA);
    rcu_periph_clock_enable(RCU_GPIOC);
    rcu_periph_clock_enable(RCU_DAC);
}

fn gpio_config() {
    const GPIO_PIN_4: u32 = bit(4);
    gpio_init(GPIOA, GPIO_MODE_AIN, GPIO_OSPEED_50MHZ, GPIO_PIN_4);
}

fn dac_config(dac: &pac::DAC) {
    dac_deinit();
    dac_trigger_source_config(&dac,DAC0, DAC_TRIGGER_SOFTWARE as u8);
    dac_trigger_enable(&dac,DAC0);
    dac_wave_mode_config(&dac,DAC0, DAC_WAVE_DISABLE as u8);
    dac_output_buffer_enable(&dac, DAC0);
    dac_enable(&dac, DAC0);
}

// The reset handler
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    r0::zero_bss(&mut _sbss, &mut _ebss);
    r0::init_data(&mut _sdata, &mut _edata, &_sidata);
    main()
}

// don't compile with optimization enabled!
fn delay(mut n: u32) {
    while n != 0 {
        n -= 1;
    }
}

fn main() -> ! {
    let array: [u8; 10] = [0x00, 0x33, 0x66, 0x99, 0xcc, 0xff, 0xcc, 0x99, 0x66, 0x33];
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();

    let gpioa = dp.GPIOA.split(&mut rcu);

    longan_nano::stdout::configure(dp.USART0, gpioa.pa9, gpioa.pa10, 115_200.bps(), &mut rcu);

    sprintln!("HELLO DAC");

    let dac = dp.DAC;
    rcu_config();
    gpio_config();
    dac_config(&dac);

    let mut dac_output: u16 = 0;
    loop {
        if dac_output >= 4096 {
            dac_output = 0;
        } else {
            if dac_output < 1000 {
                dac_output += 10;
            } else {
                dac_output += 25;
            }
        }
        dac_data_set(&dac, DAC0, DAC_ALIGN_12B_R, dac_output);
        dac_software_trigger_enable(&dac, DAC0);
        //delay(0xf);
    }
}

extern "C" {
    // Boundaries of the .bss section
    static mut _ebss: u32;
    static mut _sbss: u32;

    // Boundaries of the .data section
    static mut _edata: u32;
    static mut _sdata: u32;

    // Initial values of the .data section (stored in Flash)
    static _sidata: u32;
}

// Make sure there is an abort when linking
#[cfg(target_arch = "riscv32")]
global_asm!(
    r#"
lui sp, %hi(__stacktop)
call Reset
.globl abort
abort:
  jal zero, abort
"#
);
