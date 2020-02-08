#![feature(global_asm)]
#![no_main]
#![no_std]

use panic_abort;

mod rcu;
mod gpio;
mod dac;
mod register_helpers;

use rcu::*;
use gpio::*;
use dac::*;
pub use register_helpers::*;

fn rcu_config() {
    rcu_periph_clock_enable(RCU_GPIOA);
    rcu_periph_clock_enable(RCU_GPIOC);
    rcu_periph_clock_enable(RCU_DAC);
}

fn gpio_config() {
    const GPIO_PIN_4: u32 = bit(4);
    gpio_init(GPIOA, GPIO_MODE_AIN, GPIO_OSPEED_50MHZ, GPIO_PIN_4);
}

fn dac_config() {
    dac_deinit();
    dac_trigger_source_config(DAC0, DAC_TRIGGER_SOFTWARE);
    dac_trigger_enable(DAC0);
    dac_wave_mode_config(DAC0, DAC_WAVE_DISABLE);
    dac_output_buffer_enable(DAC0);
    dac_enable(DAC0);
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
    rcu_config();
    gpio_config();
    dac_config();

    let mut dac_output: u16 = 0;
    loop {
        if dac_output >= 4096 {
            dac_output = 0;
        } else {
            dac_output += 25;
        }
        dac_data_set(DAC0, DAC_ALIGN_12B_R, dac_output);
        dac_software_trigger_enable(DAC0);
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
