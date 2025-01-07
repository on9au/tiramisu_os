#![no_std]
#![no_main]

#[cfg(feature = "bios")]
mod bios;

#[cfg(feature = "uefi")]
mod uefi;

#[cfg(feature = "test")]
mod test;

use interrupts::idt::init_idt;
use logging::{fatal, warn};
use vga_text_mode::{clear_screen, println};

pub fn boot_main() -> ! {
    #[cfg(feature = "test")]
    {
        test::test_main();
        loop {
            unsafe { core::arch::asm!("hlt") }
        }
    }

    #[cfg(not(feature = "test"))]
    main();
}

fn main() -> ! {
    clear_screen!();
    println!("Hello World from Tiramisu Bootloader!");

    init_idt();

    x86_64::instructions::interrupts::int3();

    warn!("We are hanging here...");

    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn eh_personality() {}

#[cfg(not(feature = "test"))]
#[panic_handler]
fn panic_fmt(info: &core::panic::PanicInfo) -> ! {
    fatal!("[PANIC] {}", info.message());
    if let Some(location) = info.location() {
        fatal!("File: '{}:{}'", location.file(), location.line());
    } else {
        fatal!("File: Unavailable");
    }

    fatal!("Kernel Panic! We are hanging here...");

    loop {
        // CPU power is precious, let's save some by halting the CPU
        unsafe { core::arch::asm!("hlt") }
    }
}
