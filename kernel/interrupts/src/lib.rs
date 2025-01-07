#![no_std]
#![feature(abi_x86_interrupt)]

pub mod idt;

#[cfg(feature = "test")]
mod test;