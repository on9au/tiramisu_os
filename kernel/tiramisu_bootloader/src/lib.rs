#![no_std]
#![no_main]

mod bios;

use vga_text_mode::println;

pub fn boot_main() -> ! {
    #[cfg(feature = "test")]
    {
        tests::test_main();
        loop {
            unsafe { core::arch::asm!("hlt") }
        }
    }

    #[cfg(not(feature = "test"))]
    main();
}

fn main() -> ! {
    println!("Hello World from Tiramisu Bootloader!");

    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}

#[cfg(feature = "test")]
mod tests {
    use test_system::declare_tests;
    use vga_text_mode::println;

    #[cfg(feature = "test")]
    pub fn test_runner(tests: &[&dyn Fn()]) {
        println!("Running {} tests", tests.len());
        for test in tests {
            test();
        }
    }

    #[cfg(feature = "test")]
    pub fn test_main() {
        test_runner(&TESTS);
    }

    declare_tests! {
        test_example => {
            assert_eq!(1 + 1, 2);
        },
        another_test => {
            assert_eq!(2 + 2, 4);
        },
    }
}
