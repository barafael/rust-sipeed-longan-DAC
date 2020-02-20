#![no_std]
#![no_main]

use panic_halt as _;

use gd32vf103xx_hal as hal;
use gd32vf103xx_hal::prelude::*;
use hal::pac;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    dp.RCU.apb1en.modify(|_, w| w.dacen().bit(true));
    dp.RCU.ahben.modify(|_, w| w.dma1en().bit(true));
    dp.RCU.apb1en.modify(|_, w| w.timer5en().bit(true));

    let _rcu = dp
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();

    dp.DAC.ctl.write(|w| unsafe { w.dtsel0().bits(0b111) });
    dp.DAC.ctl.write(|w| w.dten0().set_bit());
    dp.DAC.ctl.write(|w| unsafe { w.dwm0().bits(0) });
    dp.DAC.ctl.write(|w| w.den0().set_bit());
    dp.DAC.ctl.write(|w| w.dboff0().clear_bit());
    dp.DAC.ctl.write(|w| w.den0().set_bit());

    let mut dac_output: u16 = 0;
    loop {
        if dac_output >= 4096 {
            dac_output = 0;
        } else if dac_output < 1000 {
                dac_output += 10;
        } else {
                dac_output += 25;
        }
        dp.DAC
            .dac0_r12dh
            .write(|w| unsafe { w.bits(dac_output as u32) });
        dp.DAC.swt.write(|w| w.swtr0().set_bit());
    }
}
