#![no_std]
#![no_main]

mod bios;

#[cfg(feature = "test")]
mod test;

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
    println!("We are hanging here...");

    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}

#[no_mangle]
pub extern "C" fn eh_personality() {}

#[cfg(not(feature = "test"))]
#[panic_handler]
fn panic_fmt(info: &core::panic::PanicInfo) -> ! {
    println!("Panic!");
    if let Some(location) = info.location() {
        println!("File: '{}:{}'", location.file(), location.line());
    } else {
        println!("File: Unavailable");
    }

    println!("{}", info.message());

    println!("Kernel Panic! We are hanging here...");

    loop {
        // CPU power is precious, let's save some by halting the CPU
        unsafe { core::arch::asm!("hlt") }
    }
}
