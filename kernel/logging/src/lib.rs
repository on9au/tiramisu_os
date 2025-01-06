#![no_std]

use core::fmt;

#[doc(hidden)]
pub fn _log(args: fmt::Arguments) {
    use core::fmt::Write;
    use uart_16550::SERIAL1;
    use vga_text_mode::WRITER;

    SERIAL1.lock().write_fmt(args).unwrap();
    WRITER.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! trace {
    () => ($crate::_log(format_args!("[TRACE] \n")));
    ($($arg:tt)*) => ($crate::_log(format_args!("[TRACE] {}\n", format_args!($($arg)*))));
}

#[macro_export]
macro_rules! debug {
    () => ($crate::_log(format_args!("[DEBUG] \n")));
    ($($arg:tt)*) => ($crate::_log(format_args!("[DEBUG] {}\n", format_args!($($arg)*))));
}

#[macro_export]
macro_rules! info {
    () => ($crate::_log(format_args!("[INFO ] \n")));
    ($($arg:tt)*) => ($crate::_log(format_args!("[INFO ] {}\n", format_args!($($arg)*))));
}

#[macro_export]
macro_rules! warn {
    () => ($crate::_log(format_args!("[WARN ] \n")));
    ($($arg:tt)*) => ($crate::_log(format_args!("[WARN ] {}\n", format_args!($($arg)*))));
}

#[macro_export]
macro_rules! error {
    () => ($crate::_log(format_args!("[ERROR] \n")));
    ($($arg:tt)*) => ($crate::_log(format_args!("[ERROR] {}\n", format_args!($($arg)*))));
}

#[macro_export]
macro_rules! fatal {
    () => ($crate::_log(format_args!("[FATAL] \n")));
    ($($arg:tt)*) => ($crate::_log(format_args!("[FATAL] {}\n", format_args!($($arg)*))));
}