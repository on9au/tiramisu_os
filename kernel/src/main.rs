//! # Tiramisu Kernel
//!
//! Entry point for the Tiramisu kernel.

#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    kcore::init();
    services::init();
    userland::init();

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // core::log::kernel_log("KERNEL PANIC: ");
    // core::log::kernel_log(&format!("{info}"));
    loop {
        unsafe { core::arch::asm!("hlt") };
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn eh_personality() {}
