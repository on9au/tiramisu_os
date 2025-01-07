#![no_std]
#![feature(abi_x86_interrupt)]

pub mod idt;
pub mod gdt;
#[cfg(feature = "test")]
pub mod test;

use pic8259::ChainedPics;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub fn init() {
    gdt::init_gdt();
    idt::init_idt();
    unsafe { crate::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}