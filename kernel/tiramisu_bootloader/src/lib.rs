#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub fn boot_main() -> ! {
    #[cfg(test)]
    {
        test_main();
    }

    println!("Hello World from Tiramisu Bootloader!");

    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}

mod bios;

use vga_text_mode::println;

pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Kernel is running tests...");
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
