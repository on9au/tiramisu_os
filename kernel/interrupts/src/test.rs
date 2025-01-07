// Define the tests here
test_system::declare_tests! {
    test_breakpoint_exception => {
        x86_64::instructions::interrupts::int3();
    },
}
