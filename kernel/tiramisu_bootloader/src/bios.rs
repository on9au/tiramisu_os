use uart_16550::serial_println;

use crate::boot_main;

/// Entry point for x86_64 BIOS after assembly bootloader
#[no_mangle]
pub extern "C" fn rust_bios_entry() -> ! {
    // ATTENTION: we have a very small stack and no guard page

    // Do BIOS specific initialization here
    serial_println!("Rust Entry from x86_64 BIOS");

    boot_main()
}
