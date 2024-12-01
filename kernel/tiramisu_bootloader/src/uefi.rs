use crate::boot_main;

/// Entry point for UEFI after assembly bootloader
#[no_mangle]
#[link_section = ".init.text"]
pub extern "C" fn rust_uefi_entry() -> ! {
    // ATTENTION: we have a very small stack and no guard page

    // Do UEFI specific initialization here
    // serial_println!("Rust Entry from UEFI");

    boot_main()
}
