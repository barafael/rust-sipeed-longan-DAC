use crate::register_helpers::*;

/* RCU constants */
const RCU_AHB1_BUS_BASE: u32 = (0x4001_8000);
const RCU_BASE: u32 = RCU_AHB1_BUS_BASE + 0x0000_9000;
const RCU: u32 = RCU_BASE;

const APB1EN_REG_OFFSET: u32 = 0x1c;
const APB2EN_REG_OFFSET: u32 = 0x18;

pub(crate) const RCU_GPIOA: u32 = rcu_regidx_bit(APB2EN_REG_OFFSET, 2);
pub(crate) const RCU_GPIOC: u32 = rcu_regidx_bit(APB2EN_REG_OFFSET, 4);

pub(crate) const RCU_DAC: u32 = rcu_regidx_bit(APB1EN_REG_OFFSET, 29);

pub const fn rcu_regidx_bit(regidx: u32, bitpos: u32) -> u32 {
    ((regidx << 6) as u32) | bitpos
}

pub fn rcu_periph_clock_enable(periph: u32) {
    set_bit(rcu_reg_val(periph), bit(rcu_bit_pos(periph)));
}

pub fn rcu_periph_reset_enable(periph_reset: u32) {
    set_bit(rcu_reg_val(periph_reset), bit(rcu_bit_pos(periph_reset)));
}

pub fn rcu_periph_reset_disable(periph_reset: u32) {
    reset_bit(rcu_reg_val(periph_reset), bit(rcu_bit_pos(periph_reset)));
}

fn rcu_bit_pos(val: u32) -> u32 {
    (val & 0x1f) as u32
}

fn rcu_reg_val(periph: u32) -> *mut u32 {
    reg32(RCU + ((periph >> 6) as u32))
}
