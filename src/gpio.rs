use crate::register_helpers::*;

/* GPIO constants */
pub(crate) const GPIO_MODE_AIN: u32 = 0x00;
const GPIO_MODE_IPU: u32 = 0x48;
const GPIO_MODE_IPD: u32 = 0x28;

pub(crate) const GPIO_OSPEED_50MHZ: u32 = 0x03;

const APB2_BUS_BASE: u32 = 0x4001_0000;
const GPIO_BASE: u32 = APB2_BUS_BASE + 0x0000_0800;
pub(crate) const GPIOA: u32 = GPIO_BASE + 0x0000_0000;

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

fn gpio_ctl0(gpiox: u32) -> *mut u32 {
    reg32(gpiox + 0x00)
}

pub fn gpio_init(gpio_periph: u32, mode: u32, speed: u32, pin: u32) {
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
                }
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
                }
                GPIO_MODE_IPU => {
                    set_register(gpio_bop(gpio_periph), (1 << i) & pin);
                }
                _ => (),
            }
            set_register(gpio_ctl1(gpio_periph), reg);
        }
    }
}
