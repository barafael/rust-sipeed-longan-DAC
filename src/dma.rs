use crate::register_helpers::*;

const fn chctl_pwidth(regval: u32) -> u32 {
    bits(8, 9) & (regval << 8)
}

const fn chctl_mwidth(regval: u32) -> u32 {
    bits(10, 11) & (regval << 10)
}

const fn chctl_prio(regval: u32) -> u32 {
    bits(12, 13) & (regval << 12)
}

pub const DMA_INTF_GIF: u32 = bit(0);
pub const DMA_INTF_FTFIF: u32 = bit(1);
pub const DMA_INTF_HTFIF: u32 = bit(2);
pub const DMA_INTF_ERRIF: u32 = bit(3);

pub const DMA_PERIPHERAL_WIDTH_8BIT: u32 = chctl_pwidth(0);

pub const DMA_MEMORY_WIDTH_8BIT: u32 = chctl_mwidth(0);

pub const DMA_PRIORITY_ULTRA_HIGH: u32 = chctl_prio(3);

pub const DMA_PERIPH_INCREASE_DISABLE: u8 = 0x0;
pub const DMA_PERIPH_INCREASE_ENABLE: u8 = 0x1;

pub const DMA_MEMORY_INCREASE_DISABLE: u8 = 0x0;
pub const DMA_MEMORY_INCREASE_ENABLE: u8 = 0x1;

pub const DMA_PERIPHERAL_TO_MEMORY: u8 = 0x0;
pub const DMA_MEMORY_TO_PERIPHERAL: u8 = 0x1;

const DMA_CHXCTL_PNAGA: u32 = bit(6);
const DMA_CHXCTL_MNAGA: u32 = bit(7);

const AHB1_BUS_BASE: u32 = 0x4001_8000;
const DMA_BASE: u32 = AHB1_BUS_BASE + 0x0000_8000;

pub const DMA1: u32 = DMA_BASE + 0x0400;

pub struct DmaParameters {
    pub periph_addr: u32,  // peripheral base address
    pub periph_width: u32, // transfer data size of peripheral
    pub memory_addr: u32,  // memory base address
    pub memory_width: u32, // transfer data size of memory
    pub number: u32,       // channel transfer number
    pub priority: u32,     // channel priority number
    pub periph_inc: u8,    // peripheral increasing mode
    pub memory_inc: u8,    // memory increasing mode
    pub direction: u8,     // channel data transfer direction
}

const fn dma_chpaddr(dma: u32, channel: &DmaChannel) -> *mut u32 {
    reg32((dma + 0x10) + 0x14 * (*channel as u32))
}

const fn dma_chmaddr(dma: u32, channel: &DmaChannel) -> *mut u32 {
    reg32((dma + 0x14) + 0x14 * (*channel as u32))
}

const fn dma_chcnt(dma: u32, channel: &DmaChannel) -> *mut u32 {
    reg32((dma + 0x0) + 0x14 * (*channel as u32))
}

const fn dma_chctl(dma: u32, channel: &DmaChannel) -> *mut u32 {
    reg32((dma + 0x08) + 0x14 * (*channel as u32))
}

const DMA_CHXCTL_PWIDTH: u32 = bits(8, 9);
const DMA_CHXCTL_MWIDTH: u32 = bits(10, 11);
const DMA_CHXCTL_PRIO: u32 = bits(12, 13);

const DMA_CHXCNT_CNT: u32 = bits(0, 15);
const DMA_CHANNEL_CNT_MASK: u32 = DMA_CHXCNT_CNT;

const DMA_CHXCTL_CHEN: u32 = bit(1);
const DMA_CHXCTL_DIR: u32 = bit(4);
const DMA_CHXCTL_CMEN: u32 = bit(5);

#[derive(Copy, Clone)]
pub enum DmaChannel {
    DmaCh0 = 0,
    DmaCh1,
    DmaCh2,
    DmaCh3,
    DmaCh4,
    DmaCh5,
    DmaCh6,
}

pub fn dma_init(dma_periph: u32, channelx: &DmaChannel, init_struct: &DmaParameters) -> Option<()> {
    if dma_periph_and_channel_check(dma_periph, channelx).is_none() {
        return None;
    }
    set_register(dma_chpaddr(dma_periph, channelx), init_struct.periph_addr);
    set_register(dma_chmaddr(dma_periph, channelx), init_struct.memory_addr);

    set_register(
        dma_chcnt(dma_periph, channelx),
        init_struct.number & DMA_CHANNEL_CNT_MASK,
    );

    let mut ctl: u32 = read_register(dma_chctl(dma_periph, channelx));
    ctl &= !(DMA_CHXCTL_PWIDTH | DMA_CHXCTL_MWIDTH | DMA_CHXCTL_PRIO);
    ctl |= (init_struct.periph_width | init_struct.memory_width | init_struct.priority);
    set_register(dma_chctl(dma_periph, channelx), ctl);

    if init_struct.periph_inc == DMA_PERIPH_INCREASE_ENABLE {
        set_bits(dma_chctl(dma_periph, channelx), DMA_CHXCTL_PNAGA);
    } else {
        reset_bits(dma_chctl(dma_periph, channelx), DMA_CHXCTL_PNAGA);
    }
    if init_struct.memory_inc == DMA_MEMORY_INCREASE_ENABLE {
        set_bits(dma_chctl(dma_periph, channelx), DMA_CHXCTL_MNAGA);
    } else {
        reset_bits(dma_chctl(dma_periph, channelx), DMA_CHXCTL_MNAGA);
    }
    if init_struct.direction == DMA_PERIPHERAL_TO_MEMORY {
        reset_bits(dma_chctl(dma_periph, channelx), DMA_CHXCTL_DIR);
    } else {
        set_bits(dma_chctl(dma_periph, channelx), DMA_CHXCTL_DIR);
    }
    Some(())
}

pub fn dma_periph_and_channel_check(dma_periph: u32, channelx: &DmaChannel) -> Option<()> {
    return if dma_periph == DMA1 {
        match channelx {
            DmaChannel::DmaCh0
            | DmaChannel::DmaCh1
            | DmaChannel::DmaCh2
            | DmaChannel::DmaCh3
            | DmaChannel::DmaCh4 => Some(()),
            _ => None,
        }
    } else {
        Some(())
    };
}

fn dma_flag_add(flag: u32, shift: u32) -> u32 {
    flag << (4 * shift)
}

fn dma_intc(dmax: u32) -> *mut u32 {
    reg32(dmax + 0x4)
}

pub fn dma_flag_clear(dma_periph: u32, channelx: &DmaChannel, flag: u32) {
    set_bits(dma_intc(dma_periph), dma_flag_add(flag, (*channelx as u32)));
}

pub fn dma_circulation_enable(dma_periph: u32, channelx: &DmaChannel) -> Option<()> {
    if dma_periph_and_channel_check(dma_periph, channelx).is_none() {
        return None
    }
    set_bits(dma_chctl(dma_periph, channelx), DMA_CHXCTL_CMEN);
    Some(())
}

pub fn dma_channel_enable(dma_periph: u32, channelx: &DmaChannel) -> Option<()> {
    if dma_periph_and_channel_check(dma_periph, channelx).is_none() {
        return None
    }
    set_bits(dma_chctl(dma_periph, channelx), DMA_CHXCTL_CHEN);
    Some(())
}

