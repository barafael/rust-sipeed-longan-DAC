#![feature(global_asm)]
#![no_main]
#![no_std]

use panic_abort;

/* GPIO constants */
const GPIO_MODE_AIN: u32 = 0x00;
const GPIO_MODE_IPU: u32 = 0x48;
const GPIO_MODE_IPD: u32 = 0x28;

const GPIO_OSPEED_50MHZ: u32 = 0x03;

const APB2_BUS_BASE: u32 = 0x4001_0000;
const GPIO_BASE: u32 = APB2_BUS_BASE + 0x0000_0800;
const GPIOA: u32 = GPIO_BASE + 0x0000_0000;

/* RCU constants */
const RCU_AHB1_BUS_BASE: u32 = (0x4001_8000);
const RCU_BASE: u32 = RCU_AHB1_BUS_BASE + 0x0000_9000;
const RCU: u32 = RCU_BASE;

const APB1EN_REG_OFFSET: u32 = 0x1c;
const APB2EN_REG_OFFSET: u32 = 0x18;

const RCU_GPIOA: u32 = rcu_regidx_bit(APB2EN_REG_OFFSET, 2);
const RCU_GPIOC: u32 = rcu_regidx_bit(APB2EN_REG_OFFSET, 4);

/* DAC constants */
const APB1_BUS_BASE: u32 = 0x4000_0000;
const DAC: u32 = APB1_BUS_BASE + 0x0000_7400;

const DAC0: u32 = 0;
const DAC1: u32 = 1;

const DAC_CTL: *mut u32 = reg32(DAC + 0x0);
const DAC_SWT: *mut u32 = reg32(DAC + 0x4);

const DAC0_R12DH: *mut u32 = reg32(DAC + 0x8);
const DAC1_R12DH: *mut u32 = reg32(DAC + 0x14);

const DAC_ALIGN_12B_R: u32 = data_align(0);

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

const DAC_TRIGGER_SOFTWARE: u32 = ctl_dtsel(7);

const fn ctl_dwm(regval: u32) -> u32 {
    bits(6, 7) & (regval << 6)
}

const DAC_WAVE_DISABLE: u32 = ctl_dwm(0);

const RCU_DAC: u32 = rcu_regidx_bit(APB1EN_REG_OFFSET, 29);

const fn ctl_dtsel(reg_val: u32) -> u32 {
    bits(3, 5) & (reg_val << 3)
}

const fn rcu_regidx_bit(regidx: u32, bitpos: u32) -> u32 {
    ((regidx << 6) as u32) | bitpos
}

fn rcu_periph_clock_enable(periph: u32) {
    set_bit(rcu_reg_val(periph), bit(rcu_bit_pos(periph)));
}

fn rcu_periph_reset_enable(periph_reset: u32) {
    set_bit(rcu_reg_val(periph_reset), bit(rcu_bit_pos(periph_reset)));
}

fn rcu_periph_reset_disable(periph_reset: u32) {
    reset_bit(rcu_reg_val(periph_reset), bit(rcu_bit_pos(periph_reset)));
}

fn rcu_bit_pos(val: u32) -> u32 {
    (val & 0x1f) as u32
}

fn rcu_reg_val(periph: u32) -> *mut u32 {
    reg32(RCU + ((periph >> 6) as u32))
}

const fn bit(x: u32) -> u32 {
    (0x01 << x) as u32
}

const fn bits(start: u32, end: u32) -> u32 {
    let a = 0xffff_ffffu32 << start;
    let b = 0xffff_ffffu32 >> (31u32 - end);
    a & b
}

const fn reg32(addr: u32) -> *mut u32 {
    addr as *mut u32
}

const fn data_align(reg_val: u32) -> u32 {
    bits(0, 1) & (reg_val)
}

fn dac_deinit() {
    const APB1RST_REG_OFFSET: u32 = 0x10;
    let rcu_dacrst: u32 = rcu_regidx_bit(APB1RST_REG_OFFSET, 29u32);
    rcu_periph_reset_enable(rcu_dacrst);
    rcu_periph_reset_disable(rcu_dacrst);
}

fn dac_enable(dac_periph: u32) {
    if dac_periph == DAC0 {
        set_bit(DAC_CTL, DAC_CTL_DEN0);
    } else if dac_periph == DAC1 {
        set_bit(DAC_CTL, DAC_CTL_DEN1);
    }
}

fn dac_disable(dac_periph: u32) {
    if dac_periph == DAC0 {
        reset_bit(DAC_CTL, DAC_CTL_DEN0);
    } else if dac_periph == DAC1 {
        reset_bit(DAC_CTL, DAC_CTL_DEN1);
    }
}

fn dac_dma_enable(dac_periph: u32) {
    if dac_periph == DAC0 {
        set_bit(DAC_CTL, DAC_CTL_DDMAEN0);
    } else if dac_periph == DAC1 {
        set_bit(DAC_CTL, DAC_CTL_DDMAEN1);
    }
}

fn dac_dma_disable(dac_periph: u32) {
    if dac_periph == DAC0 {
        reset_bit(DAC_CTL, DAC_CTL_DDMAEN0);
    } else if dac_periph == DAC1 {
        reset_bit(DAC_CTL, DAC_CTL_DDMAEN1);
    }
}

fn dac_output_buffer_enable(dac_periph: u32) {
    if dac_periph == DAC0 {
        reset_bit(DAC_CTL, DAC_CTL_DBOFF0);
    } else if dac_periph == DAC1 {
        reset_bit(DAC_CTL, DAC_CTL_DBOFF1);
    }
}

fn dac_output_buffer_disable(dac_periph: u32) {
    if dac_periph == DAC0 {
        set_bit(DAC_CTL, DAC_CTL_DBOFF0);
    } else if dac_periph == DAC1 {
        set_bit(DAC_CTL, DAC_CTL_DBOFF1);
    }
}

fn dac_output_value_get(dac_periph: u32) -> u32 {
    match dac_periph {
        DAC0 => read_register(DAC0_DO),
        DAC1 => read_register(DAC1_DO),
        _ => 0,
    }
}

fn dac_data_set(dac_periph: u32, dac_align: u32, data: u16) {
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

fn dac_trigger_enable(dac_periph: u32) {
    if dac_periph == DAC0 {
        set_bit(DAC_CTL, DAC_CTL_DTEN0);
    } else if dac_periph == DAC1 {
        set_bit(DAC_CTL, DAC_CTL_DTEN1);
    }
}

fn dac_trigger_disable(dac_periph: u32) {
    if dac_periph == DAC0 {
        reset_bit(DAC_CTL, DAC_CTL_DTEN0);
    } else if dac_periph == DAC1 {
        reset_bit(DAC_CTL, DAC_CTL_DTEN1);
    }
}

fn dac_wave_mode_config(dac_periph: u32, wave_mode: u32) {
    if dac_periph == DAC0 {
        reset_bit(DAC_CTL, DAC_CTL_DWM0);
        set_bit(DAC_CTL, wave_mode);
    } else if dac_periph == DAC1 {
        reset_bit(DAC_CTL, DAC_CTL_DWM1);
        set_bit(DAC_CTL, wave_mode << DAC1_REG_OFFSET);
    }
}

fn rcu_config() {
    rcu_periph_clock_enable(RCU_GPIOA);
    rcu_periph_clock_enable(RCU_GPIOC);
    rcu_periph_clock_enable(RCU_DAC);
}

fn gpio_ctl0(gpiox: u32) -> *mut u32 {
    reg32(gpiox + 0x00)
}

fn gpio_mode_mask(n: u32) -> u32 {
    0xf << (4 * n)
}

fn gpio_mode_set(n: u32, mode: u32) -> u32 {
    mode << (4 * n)
}

fn gpio_bc(gpiox: u32) -> *mut u32 {
    reg32(gpiox + 0x14)
}

const fn gpio_bop(gpiox: u32) -> *mut u32 {
    reg32(gpiox + 0x10)
}

const fn gpio_ctl1(gpiox: u32) -> *mut u32 {
    reg32(gpiox + 0x04)
}

fn gpio_bit_set(gpio_periph: u32, pin: u32) {
    set_register(gpio_bop(gpio_periph), pin);
}

fn gpio_init(gpio_periph: u32, mode: u32, speed: u32, pin: u32) {
    let mut temp_mode = (mode & 0x0fu32) as u32;

    if 0x00 != (mode & 0x10) {
        temp_mode |= speed;
    }
    for i in 0..8u32 {
        if (1u32 << i) & pin != 0 {
            let mut reg = read_register(gpio_ctl0(gpio_periph));

            reg &= !gpio_mode_mask(i);
            reg |= gpio_mode_set(i, temp_mode);

            match mode {
                GPIO_MODE_AIN => (),
                GPIO_MODE_IPD => {
                    set_register(gpio_bc(gpio_periph), (1 << i) & pin);
                },
                GPIO_MODE_IPU => {
                    set_register(gpio_bop(gpio_periph), (1 << i) & pin);
                }
                _ => (),
            };
            set_register(gpio_ctl0(gpio_periph), reg);
        }
    }
    for i in 8..16u32 {
        if (1 << i) & pin != 0 {
            let mut reg = read_register(gpio_ctl1(gpio_periph));

            reg &= !gpio_mode_mask(i - 8);
            reg |= gpio_mode_set(i - 8, temp_mode);

            match mode {
                GPIO_MODE_AIN => (),
                GPIO_MODE_IPD => {
                    set_register(gpio_bc(gpio_periph), (1 << i) & pin);
                },
                GPIO_MODE_IPU => {
                    set_register(gpio_bop(gpio_periph), (1 << i) & pin);
                }
                _ => (),
            }
            set_register(gpio_ctl1(gpio_periph), reg);
        }
    }
}

fn gpio_config() {
    const GPIO_PIN_4: u32 = bit(4);
    gpio_init(GPIOA, GPIO_MODE_AIN, GPIO_OSPEED_50MHZ, GPIO_PIN_4);
}

fn dac_trigger_source_config(dac_periph: u32, triggersource: u32) {
    if dac_periph == DAC0 {
        reset_bit(DAC_CTL, DAC_CTL_DTSEL0);
        set_bit(DAC_CTL, triggersource);
    } else if dac_periph == DAC1 {
        reset_bit(DAC_CTL, DAC_CTL_DTSEL1);
        set_bit(DAC_CTL, triggersource << DAC1_REG_OFFSET);
    }
}

fn dac_software_trigger_enable(dac_periph: u32) {
    if dac_periph == DAC0 {
        set_bit(DAC_SWT, DAC_SWT_SWTR0);
    } else if dac_periph == DAC1 {
        set_bit(DAC_SWT, DAC_SWT_SWTR1);
    }
}

fn dac_config() {
    dac_deinit();
    dac_trigger_source_config(DAC0, DAC_TRIGGER_SOFTWARE);
    dac_trigger_enable(DAC0);
    dac_wave_mode_config(DAC0, DAC_WAVE_DISABLE);
    dac_output_buffer_enable(DAC0);
    dac_enable(DAC0);
}

fn read_register(register: *mut u32) -> u32 {
    unsafe { core::ptr::read_volatile(register) }
}

fn set_register(register: *mut u32, value: u32) {
    unsafe {
        core::ptr::write_volatile(register, value);
    }
}

fn set_bit(reg: *mut u32, bit: u32) {
    unsafe {
        let mut value = core::ptr::read_volatile(reg);
        value |= bit;
        core::ptr::write_volatile(reg, value);
    }
}

fn reset_bit(reg: *mut u32, bit: u32) {
    unsafe {
        let mut value = core::ptr::read_volatile(reg);
        value &= !bit;
        core::ptr::write_volatile(reg, value);
    }
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
            dac_output += 75;
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
