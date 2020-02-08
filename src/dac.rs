use crate::register_helpers::*;
use crate::rcu::rcu_periph_reset_disable;
use crate::rcu::rcu_periph_reset_enable;
use crate::rcu::rcu_regidx_bit;

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

pub fn dac_enable(dac_periph: u32) {
    if dac_periph == DAC0 {
        set_bit(DAC_CTL, DAC_CTL_DEN0);
    } else if dac_periph == DAC1 {
        set_bit(DAC_CTL, DAC_CTL_DEN1);
    }
}

pub fn dac_disable(dac_periph: u32) {
    if dac_periph == DAC0 {
        reset_bit(DAC_CTL, DAC_CTL_DEN0);
    } else if dac_periph == DAC1 {
        reset_bit(DAC_CTL, DAC_CTL_DEN1);
    }
}

pub fn dac_dma_enable(dac_periph: u32) {
    if dac_periph == DAC0 {
        set_bit(DAC_CTL, DAC_CTL_DDMAEN0);
    } else if dac_periph == DAC1 {
        set_bit(DAC_CTL, DAC_CTL_DDMAEN1);
    }
}

pub fn dac_dma_disable(dac_periph: u32) {
    if dac_periph == DAC0 {
        reset_bit(DAC_CTL, DAC_CTL_DDMAEN0);
    } else if dac_periph == DAC1 {
        reset_bit(DAC_CTL, DAC_CTL_DDMAEN1);
    }
}

pub fn dac_output_buffer_enable(dac_periph: u32) {
    if dac_periph == DAC0 {
        reset_bit(DAC_CTL, DAC_CTL_DBOFF0);
    } else if dac_periph == DAC1 {
        reset_bit(DAC_CTL, DAC_CTL_DBOFF1);
    }
}

pub fn dac_output_buffer_disable(dac_periph: u32) {
    if dac_periph == DAC0 {
        set_bit(DAC_CTL, DAC_CTL_DBOFF0);
    } else if dac_periph == DAC1 {
        set_bit(DAC_CTL, DAC_CTL_DBOFF1);
    }
}

pub fn dac_output_value_get(dac_periph: u32) -> u32 {
    match dac_periph {
        DAC0 => read_register(DAC0_DO),
        DAC1 => read_register(DAC1_DO),
        _ => 0,
    }
}

pub fn dac_data_set(dac_periph: u32, dac_align: u32, data: u16) {
    if dac_periph == DAC0 {
        match dac_align {
            DAC_ALIGN_12B_R => set_register(DAC0_R12DH, data as u32),
            _ => unimplemented!(),
        };
    } else if dac_periph == DAC1 {
        match dac_align {
            DAC_ALIGN_12B_R => set_register(DAC1_R12DH, data as u32),
            _ => unimplemented!(),
        }
    }
}

pub fn dac_trigger_enable(dac_periph: u32) {
    if dac_periph == DAC0 {
        set_bit(DAC_CTL, DAC_CTL_DTEN0);
    } else if dac_periph == DAC1 {
        set_bit(DAC_CTL, DAC_CTL_DTEN1);
    }
}

pub fn dac_trigger_disable(dac_periph: u32) {
    if dac_periph == DAC0 {
        reset_bit(DAC_CTL, DAC_CTL_DTEN0);
    } else if dac_periph == DAC1 {
        reset_bit(DAC_CTL, DAC_CTL_DTEN1);
    }
}

pub fn dac_wave_mode_config(dac_periph: u32, wave_mode: u32) {
    if dac_periph == DAC0 {
        reset_bit(DAC_CTL, DAC_CTL_DWM0);
        set_bit(DAC_CTL, wave_mode);
    } else if dac_periph == DAC1 {
        reset_bit(DAC_CTL, DAC_CTL_DWM1);
        set_bit(DAC_CTL, wave_mode << DAC1_REG_OFFSET);
    }
}

pub fn dac_trigger_source_config(dac_periph: u32, triggersource: u32) {
    if dac_periph == DAC0 {
        reset_bit(DAC_CTL, DAC_CTL_DTSEL0);
        set_bit(DAC_CTL, triggersource);
    } else if dac_periph == DAC1 {
        reset_bit(DAC_CTL, DAC_CTL_DTSEL1);
        set_bit(DAC_CTL, triggersource << DAC1_REG_OFFSET);
    }
}

pub fn dac_software_trigger_enable(dac_periph: u32) {
    if dac_periph == DAC0 {
        set_bit(DAC_SWT, DAC_SWT_SWTR0);
    } else if dac_periph == DAC1 {
        set_bit(DAC_SWT, DAC_SWT_SWTR1);
    }
}

pub fn dac_deinit() {
    const APB1RST_REG_OFFSET: u32 = 0x10;
    let rcu_dacrst: u32 = rcu_regidx_bit(APB1RST_REG_OFFSET, 29u32);
    rcu_periph_reset_enable(rcu_dacrst);
    rcu_periph_reset_disable(rcu_dacrst);
}
