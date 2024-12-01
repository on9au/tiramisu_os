use crate::println;
use crate::BUFFER_HEIGHT;

// Define the tests here
test_system::declare_tests! {
    test_println_simple => {
        println!("test_println_simple output");
    },

    test_println_many => {
        for _ in 0..200 {
            println!("test_println_many output");
        }
    },

    test_println_output => {
        let s = "Some test string that fits on a single line";
        println!("{}", s);
        for (i, c) in s.chars().enumerate() {
            let screen_char = crate::WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i];
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    },
}
