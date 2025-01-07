#![no_std]
#![feature(abi_x86_interrupt)]

pub mod idt;
pub mod gdt;

#[cfg(feature = "test")]
pub mod test;

pub fn init() {
    gdt::init_gdt();
    idt::init_idt();
}