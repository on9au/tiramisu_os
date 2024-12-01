use test_system::declare_tests;
use uart_16550::serial_println;
use vga_text_mode::println;

// Define the tests here
declare_tests! {
    test_example => {
        assert_eq!(1 + 1, 2);
    },
    another_test => {
        assert_eq!(2 + 2, 4);
    },
    test_fail => {
        assert_eq!(1 + 1, 3);
    },
}

/// Declare tests in a given module.
const ALL_TESTS: &[&[(&dyn Fn(), &str)]] =
    &[TESTS, vga_text_mode::test::TESTS, uart_16550::test::TESTS];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[panic_handler]
fn panic_fmt(info: &core::panic::PanicInfo) -> ! {
    serial_println!("[FAILED] Test Panicked!");
    if let Some(location) = info.location() {
        serial_println!("File: '{}:{}'", location.file(), location.line());
    } else {
        serial_println!("File: Unavailable");
    }
    serial_println!("{}", info.message());
    exit_qemu(QemuExitCode::Failed);

    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}

pub fn test_runner(tests: &[&[(&dyn Fn(), &str)]]) {
    println!("Running in test mode. Please check the serial output for test results.");
    serial_println!("Running {} tests", tests.len());
    for module in tests {
        for (test, name) in *module {
            serial_println!("Running test: {}", name);
            test();
            serial_println!("[ok]");
        }
    }
    println!("All tests passed!");
    serial_println!("All tests passed!");
    exit_qemu(QemuExitCode::Success);
}

pub fn test_main() {
    test_runner(ALL_TESTS);
}
