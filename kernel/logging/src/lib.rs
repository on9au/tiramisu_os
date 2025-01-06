#![no_std]

#[macro_export]
macro_rules! log_with_level {
    ($lvl:expr, $($arg:tt)*) => {{
        let msg_str = format_args!($($arg)*);
        uart_16550::serial_println!("[{:5}] {}", $lvl, msg_str);
        vga_text_mode::println!("[{:5}] {}", $lvl, msg_str);
    }};
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => ($crate::log_with_level!("TRACE", $($arg)*));
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ($crate::log_with_level!("DEBUG", $($arg)*));
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ($crate::log_with_level!("INFO", $($arg)*));
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ($crate::log_with_level!("WARN", $($arg)*));
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ($crate::log_with_level!("ERROR", $($arg)*));
}