use logging::info;

use crate::boot_main;

/// Entry point for UEFI after assembly bootloader
#[no_mangle]
#[link_section = ".init.text"]
pub extern "C" fn rust_uefi_entry() -> ! {
    // ATTENTION: we have a very small stack and no guard page

    // Do UEFI specific initialization here
    info!("Entry from x86_64 UEFI");

    boot_main()
}
