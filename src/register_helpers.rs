pub const fn bit(x: u32) -> u32 {
    (0x01 << x) as u32
}

pub const fn bits(start: u32, end: u32) -> u32 {
    let a = 0xffff_ffffu32 << start;
    let b = 0xffff_ffffu32 >> (31u32 - end);
    a & b
}

pub const fn reg32(addr: u32) -> *mut u32 {
    addr as *mut u32
}

pub const fn data_align(reg_val: u32) -> u32 {
    bits(0, 1) & (reg_val)
}

pub fn read_register(register: *mut u32) -> u32 {
    unsafe { core::ptr::read_volatile(register) }
}

pub fn set_register(register: *mut u32, value: u32) {
    unsafe {
        core::ptr::write_volatile(register, value);
    }
}

pub fn set_bits(reg: *mut u32, bits: u32) {
    unsafe {
        let mut value = core::ptr::read_volatile(reg);
        value |= bits;
        core::ptr::write_volatile(reg, value);
    }
}

pub fn reset_bits(reg: *mut u32, bit: u32) {
    unsafe {
        let mut value = core::ptr::read_volatile(reg);
        value &= !bit;
        core::ptr::write_volatile(reg, value);
    }
}
