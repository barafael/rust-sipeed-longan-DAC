use crate::rcu::rcu_periph_reset_disable;
use crate::rcu::rcu_periph_reset_enable;
use crate::rcu::rcu_regidx_bit;
use crate::register_helpers::*;

use gd32vf103xx_hal as hal;
use gd32vf103xx_hal::pac;

/* DAC constants */
// Some of these have to be crate-public. A better API would hide that.
const APB1_BUS_BASE: u32 = 0x4000_0000;
const DAC: u32 = APB1_BUS_BASE + 0x0000_7400;

pub(crate) const DAC0: u32 = 0;
const DAC1: u32 = 1;

const DAC_CTL: *mut u32 = reg32(DAC + 0x0);
const DAC_SWT: *mut u32 = reg32(DAC + 0x4);

const DAC0_R12DH: *mut u32 = reg32(DAC + 0x8);
const DAC1_R12DH: *mut u32 = reg32(DAC + 0x14);

pub(crate) const DAC_ALIGN_12B_R: u32 = data_align(0);
pub(crate) const DAC_ALIGN_12B_L: u32 = data_align(1);
pub(crate) const DAC_ALIGN_8B_R: u32 = data_align(2);

const DAC0_DO: *mut u32 = reg32(DAC + 0x2c);
const DAC1_DO: *mut u32 = reg32(DAC + 0x30);

const DAC_CTL_DEN0: u32 = bit(0);
const DAC_CTL_DEN1: u32 = bit(16);

const DAC_CTL_DBOFF0: u32 = bit(1);
const DAC_CTL_DBOFF1: u32 = bit(17);

const DAC_CTL_DTEN0: u32 = bit(2);
const DAC_CTL_DTEN1: u32 = bit(18);

const DAC_CTL_DDMAEN0: u32 = bit(12);
const DAC_CTL_DDMAEN1: u32 = bit(28);

const DAC_CTL_DWM0: u32 = bits(6, 7);
const DAC_CTL_DWM1: u32 = bits(22, 23);

const DAC1_REG_OFFSET: u32 = 16;

const DAC_CTL_DTSEL0: u32 = bits(3, 5);
const DAC_CTL_DTSEL1: u32 = bits(19, 21);

const DAC_SWT_SWTR0: u32 = bit(0);
const DAC_SWT_SWTR1: u32 = bit(1);

pub(crate) const DAC_TRIGGER_SOFTWARE: u32 = ctl_dtsel(7);

const fn ctl_dwm(regval: u32) -> u32 {
    bits(6, 7) & (regval << 6)
}

pub(crate) const DAC_WAVE_DISABLE: u32 = ctl_dwm(0);

const fn ctl_dtsel(reg_val: u32) -> u32 {
    bits(3, 5) & (reg_val << 3)
}

pub fn dac_enable(dac: &pac::DAC, dac_periph: u32) {
    if dac_periph == DAC0 {
        unsafe {
            dac.ctl.write(|w| w.den0().set_bit());
        }
    } else if dac_periph == DAC1 {
        unsafe {
            dac.ctl.write(|w| w.den1().set_bit());
        }
    }
}

pub fn dac_disable(dac: &pac::DAC, dac_periph: u32) {
    if dac_periph == DAC0 {
        unsafe {
            dac.ctl.write(|w| w.den0().set_bit());
        }
    } else if dac_periph == DAC1 {
        unsafe {
            dac.ctl.write(|w| w.den1().set_bit());
        }
    }
}

pub fn dac_dma_enable(dac: &pac::DAC, dac_periph: u32) {
    if dac_periph == DAC0 {
        unsafe { dac.ctl.write(|w| w.ddmaen0().set_bit()); }
    } else if dac_periph == DAC1 {
        unsafe { dac.ctl.write(|w| w.ddmaen1().set_bit()); }
    }
}

pub fn dac_dma_disable(dac: &pac::DAC, dac_periph: u32) {
    if dac_periph == DAC0 {
        unsafe { dac.ctl.write(|w| w.ddmaen0().clear_bit()); }
    } else if dac_periph == DAC1 {
        unsafe { dac.ctl.write(|w| w.ddmaen1().clear_bit()); }
    }
}

pub fn dac_output_buffer_enable(dac: &pac::DAC, dac_periph: u32) {
    if dac_periph == DAC0 {
        unsafe { dac.ctl.write(|w| w.dboff0().clear_bit()); }
    } else if dac_periph == DAC1 {
        unsafe { dac.ctl.write(|w| w.dboff1().clear_bit()); }
    }
}

pub fn dac_output_buffer_disable(dac: &pac::DAC, dac_periph: u32) {
    if dac_periph == DAC0 {
        unsafe {
            dac.ctl.write(|w| w.dboff0().set_bit());
        }
    } else if dac_periph == DAC1 {
        dac.ctl.write(|w| w.dboff1().set_bit());
    }
}

pub fn dac_output_value_get(dac: &pac::DAC, dac_periph: u32) -> u32 {
    match dac_periph {
        DAC0 => dac.dac0_do.read().bits(),
        DAC1 => dac.dac1_do.read().bits(),
        _ => 0,
    }
}

pub fn dac_data_set(dac: &pac::DAC, dac_periph: u32, dac_align: u32, data: u16) {
    if dac_periph == DAC0 {
        match dac_align {
            DAC_ALIGN_12B_R => unsafe { dac.dac0_r12dh.write(|w| w.bits(data as u32)) },
            DAC_ALIGN_12B_L => unsafe { dac.dac0_l12dh.write(|w| w.bits(data as u32)) },
            DAC_ALIGN_8B_R => unsafe { dac.dac0_r8dh.write(|w| w.bits(data as u32)) },
            _ => unimplemented!(),
        };
    } else if dac_periph == DAC1 {
        match dac_align {
            DAC_ALIGN_12B_R => unsafe { dac.dac1_r12dh.write(|w| w.bits(data as u32)) },
            DAC_ALIGN_12B_L => unsafe { dac.dac1_l12dh.write(|w| w.bits(data as u32)) },
            DAC_ALIGN_8B_R => unsafe { dac.dac1_r8dh.write(|w| w.bits(data as u32)) },
            _ => unimplemented!(),
        }
    }
}

pub fn dac_trigger_enable(dac: &pac::DAC, dac_periph: u32) {
    if dac_periph == DAC0 {
        unsafe { dac.ctl.write(|w| w.dten0().set_bit()); }
    } else if dac_periph == DAC1 {
        unsafe { dac.ctl.write(|w| w.dten1().set_bit()); }
    }
}

pub fn dac_trigger_disable(dac: &pac::DAC, dac_periph: u32) {
    if dac_periph == DAC0 {
        unsafe { dac.ctl.write(|w| w.dten0().clear_bit()); }
    } else if dac_periph == DAC1 {
        unsafe { dac.ctl.write(|w| w.dten1().clear_bit()); }
    }
}

pub fn dac_wave_mode_config(dac: &pac::DAC, dac_periph: u32, wave_mode: u8) {
    if dac_periph == DAC0 {
        unsafe {
            dac.ctl.write(|w| w.dwm0().bits(wave_mode));
        }
    } else if dac_periph == DAC1 {
        unsafe {
            dac.ctl.write(|w| w.dwm1().bits(wave_mode));
        }
    }
}

pub fn dac_trigger_source_config(dac: &pac::DAC, dac_periph: u32, triggersource: u8) {
    if dac_periph == DAC0 {
        unsafe {
            dac.ctl.write(|w| w.dtsel0().bits(0));
            dac.ctl.write(|w| w.dtsel0().bits(triggersource));
        }
    } else if dac_periph == DAC1 {
        unsafe {
            dac.ctl.write(|w| w.dtsel1().bits(0));
            dac.ctl.write(|w| w.dtsel1().bits(triggersource));
        }
    }
}

pub fn dac_software_trigger_enable(dac: &pac::DAC, dac_periph: u32) {
    if dac_periph == DAC0 {
        unsafe { dac.swt.write(|w| w.swtr0().set_bit()); }
    } else if dac_periph == DAC1 {
        unsafe { dac.swt.write(|w| w.swtr1().set_bit()); }
    }
}

pub fn dac_deinit() {
    const APB1RST_REG_OFFSET: u32 = 0x10;
    let rcu_dacrst: u32 = rcu_regidx_bit(APB1RST_REG_OFFSET, 29u32);
    rcu_periph_reset_enable(rcu_dacrst);
    rcu_periph_reset_disable(rcu_dacrst);
}
