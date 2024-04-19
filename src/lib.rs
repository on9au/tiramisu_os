#![no_std]

#[no_mangle]
pub extern "C" fn rust_entry() -> ! {
    // ATTENTION: we have a very small stack and no guard page

    let hello = b"Hello World!";
    let color_byte = 0x1f; // white foreground, blue background

    let mut hello_colored = [color_byte; 24];
    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i*2] = *char_byte;
    }

    // write `Hello World!` to the center of the VGA text buffer
    let buffer_ptr = (0xb8000 + 1988) as *mut _;
    unsafe { *buffer_ptr = hello_colored };

    loop{}
}

#[no_mangle]
pub extern fn eh_personality() {}

#[panic_handler]
fn panic_fmt(_info: &core::panic::PanicInfo) -> ! {
    // Implement your custom panic_fmt function here
    // Print panic information or take any other desired action
    loop {}
}