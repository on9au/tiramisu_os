use vga_text_mode::println;

use crate::boot_main;

#[no_mangle]
pub extern "C" fn rust_entry() -> ! {
    // ATTENTION: we have a very small stack and no guard page

    // Do BIOS specific initialization here

    boot_main()
}

#[no_mangle]
pub extern "C" fn eh_personality() {}

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

// #[test_case]
// fn trivial_assertion() {
//     print!("trivial assertion... ");
//     assert_eq!(1, 1);
//     println!("[ok]");
// }
