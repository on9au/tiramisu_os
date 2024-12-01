use uart_16550::serial_println;

use crate::boot_main;

#[no_mangle]
pub extern "C" fn rust_entry() -> ! {
    // ATTENTION: we have a very small stack and no guard page

    // Do BIOS specific initialization here
    serial_println!("Rust Entry from x86_64 BIOS");

    boot_main()
}
