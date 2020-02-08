use crate::register_helpers::*;

const APB1_BUS_BASE: u32 = 0x4000_0000;

const TIMER_BASE: u32 = APB1_BUS_BASE + 0x0;

pub(crate) const TIMER0: u32 = TIMER_BASE + 0x0001_2C00;
pub(crate) const TIMER1: u32 = TIMER_BASE + 0x0000_0000;
pub(crate) const TIMER2: u32 = TIMER_BASE + 0x0000_0400;
pub(crate) const TIMER3: u32 = TIMER_BASE + 0x0000_0800;
pub(crate) const TIMER4: u32 = TIMER_BASE + 0x0000_0C00;
pub(crate) const TIMER5: u32 = TIMER_BASE + 0x0000_1000;
pub(crate) const TIMER6: u32 = TIMER_BASE + 0x0000_1400;

const TIMER_CTL1_MMC: u32 = bits(4,6);

pub const TIMER_TRI_OUT_SRC_RESET: u32 = ctl1_mmc(0);
pub const TIMER_TRI_OUT_SRC_ENABLE: u32 = ctl1_mmc(1);
pub const TIMER_TRI_OUT_SRC_UPDATE: u32 = ctl1_mmc(2);
pub const TIMER_TRI_OUT_SRC_CH0: u32 = ctl1_mmc(3);
pub const TIMER_TRI_OUT_SRC_O0CPRE: u32 = ctl1_mmc(4);
pub const TIMER_TRI_OUT_SRC_O1CPRE: u32 = ctl1_mmc(5);
pub const TIMER_TRI_OUT_SRC_O2CPRE: u32 = ctl1_mmc(6);
pub const TIMER_TRI_OUT_SRC_O3CPRE: u32 = ctl1_mmc(7);

const TIMER_SWEVG_UPG: u32 = bit(0);

const TIMER_CTL0_CEN: u32 = bit(0);
const TIMER_CTL0_UPDIS: u32 = bit(1);
const TIMER_CTL0_UPS: u32 = bit(2);
const TIMER_CTL0_SPM: u32 = bit(3);
const TIMER_CTL0_DIR: u32 = bit(4);
const TIMER_CTL0_CAM: u32 = bits(5,6);
const TIMER_CTL0_ARSE: u32 = bit(7);
const TIMER_CTL0_CKDIV: u32 = bits(8,9);

pub const TIMER_PSC_RELOAD_NOW: u32 = TIMER_SWEVG_UPG;
pub const TIMER_PSC_RELOAD_UPDATE: u32 = 0x0000_0000;

const fn ctl1_mmc(regval: u32) -> u32 {
    bits(4, 6) & (regval << 4)
}

const fn timer_psc(timerx: u32) -> *mut u32 {
    reg32(timerx + 0x28)
}

const fn timer_swevg(timerx: u32) -> *mut u32 {
    reg32(timerx + 0x14)
}

pub fn timer_prescaler_config(timer_periph: u32, prescaler: u16, pscreload: u32) {
    set_register(timer_psc(timer_periph), prescaler as u32);

    if pscreload == TIMER_PSC_RELOAD_NOW {
        set_bits(timer_swevg(timer_periph), TIMER_SWEVG_UPG);
    }
}

fn timer_car(timerx: u32) -> *mut u32 {
    reg32(timerx + 0x2c)
}

pub fn timer_autoreload_value_config(timer_periph: u32, autoreload: u16) {
    set_register(timer_car(timer_periph), autoreload as u32);
}

pub fn timer_master_output_trigger_source_select(timer_periph: u32, outrigger: u32) {
    reset_bits(timer_ctl1(timer_periph), TIMER_CTL1_MMC);
    set_bits(timer_ctl1(timer_periph), outrigger);
}

fn timer_ctl0(timerx: u32) -> *mut u32 {
    reg32(timerx + 0x00)
}

fn timer_ctl1(timerx: u32) -> *mut u32 {
    reg32(timerx + 0x04)
}

pub fn timer_enable(timer_periph: u32) {
    set_bits(timer_ctl0(timer_periph), TIMER_CTL0_CEN);
}
