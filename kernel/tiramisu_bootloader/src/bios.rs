use logging::info;

use crate::boot_main;

/// Entry point for x86_64 BIOS after assembly bootloader
#[unsafe(no_mangle)]
pub extern "C" fn rust_bios_entry(multiboot_info_ptr: usize) -> ! {
    // ATTENTION: we have a very small stack and no guard page

    // Do BIOS specific initialization here
    info!("Entry from x86_64 BIOS. Multiboot info at: {:#X}", multiboot_info_ptr);

    boot_main()
}
