#![feature(global_asm)]
#![no_main]
#![no_std]

use panic_abort;

mod dac;
mod dma;
mod gpio;
mod timer;
mod rcu;
mod register_helpers;

use crate::dma::DmaChannel::DmaCh2;
use crate::DmaChannel::DmaCh4;
use dac::*;
use dma::*;
use timer::*;
use gpio::*;
use rcu::*;
pub use register_helpers::*;

const SIZE: usize = 10;
const ARRAY: [u16; SIZE] = [20, 40, 60, 80, 100, 120, 140, 160, 180, 200];

fn rcu_config() {
    rcu_periph_clock_enable(RCU_GPIOA);
    rcu_periph_clock_enable(RCU_DMA1);
    rcu_periph_clock_enable(RCU_DAC);
    rcu_periph_clock_enable(RCU_TIMER5);
}

fn gpio_config() {
    const GPIO_PIN_4: u32 = bit(4);
    gpio_init(GPIOA, GPIO_MODE_AIN, GPIO_OSPEED_50MHZ, GPIO_PIN_4);
}

fn dac_config() {
    dac_deinit();
    dac_trigger_source_config(DAC0, DAC_TRIGGER_T5_TRGO);
    dac_trigger_enable(DAC0);
    dac_wave_mode_config(DAC0, DAC_WAVE_DISABLE);
    dac_output_buffer_enable(DAC0);

    dac_enable(DAC0);
    dac_dma_enable(DAC0);
}

fn timer5_config() {
    timer_prescaler_config(TIMER5, 0xf, TIMER_PSC_RELOAD_UPDATE);
    timer_autoreload_value_config(TIMER5, 0xff);
    timer_master_output_trigger_source_select(TIMER5, TIMER_TRI_OUT_SRC_UPDATE);

    timer_enable(TIMER5);
}

fn dma_config() {
    dma_flag_clear(DMA1, &DmaChannel::DmaCh2, DMA_INTF_GIF);
    dma_flag_clear(DMA1, &DmaChannel::DmaCh2, DMA_INTF_FTFIF);
    dma_flag_clear(DMA1, &DmaChannel::DmaCh2, DMA_INTF_HTFIF);
    dma_flag_clear(DMA1, &DmaChannel::DmaCh2, DMA_INTF_ERRIF);

    let pointer = ARRAY.as_ptr() as u32;
    let a = DmaParameters {
        periph_addr: DAC0_R8DH_ADDRESS,
        periph_width: DMA_PERIPHERAL_WIDTH_8BIT,
        memory_addr: pointer,
        memory_width: DMA_MEMORY_WIDTH_8BIT,
        number: SIZE as u32,
        priority: DMA_PRIORITY_ULTRA_HIGH,
        periph_inc: DMA_PERIPH_INCREASE_DISABLE,
        memory_inc: DMA_MEMORY_INCREASE_ENABLE,
        direction: DMA_MEMORY_TO_PERIPHERAL,
    };

    dma_init(DMA1, &DmaChannel::DmaCh2, &a);
    dma_circulation_enable(DMA1, &DmaChannel::DmaCh2);
    dma_channel_enable(DMA1, &DmaChannel::DmaCh2);
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
    dma_config();
    dac_config();
    timer5_config();

    loop {}
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
