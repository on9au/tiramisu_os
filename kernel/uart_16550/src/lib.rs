#![no_std]

pub mod errors;

#[cfg(feature = "test")]
pub mod test;

use core::fmt::{self, Write};

use bitflags::bitflags;
use errors::Uart16550Error;
use spin::Mutex;
const COM1: u16 = 0x3F8; // COM1

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct LineStsFlags: u8 {
        const INPUT_FULL = 1;
        // 1 to 4 unknown
        const OUTPUT_EMPTY = 1 << 5;
        // 6 and 7 unknown
    }
}

pub struct Uart16550 {
    base_addr: u16,
}

impl Uart16550 {
    /// Creates a new UART instance with the given base address.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it allows creating a UART instance with any base address,
    /// which could lead to undefined behavior if the address is incorrect or not properly mapped.
    pub const unsafe fn new(base_addr: u16) -> Self {
        Uart16550 { base_addr }
    }

    /// Port Data
    ///
    /// Read/Write: 8 bits
    fn port_data(&self) -> u16 {
        self.base_addr
    }

    /// Port Interrupt Enable
    ///
    /// Write: 8 bits
    fn port_interrupt_enable(&self) -> u16 {
        self.base_addr + 1
    }

    /// Port FIFO Control
    ///
    /// Write: 8 bits
    fn port_fifo_control(&self) -> u16 {
        self.base_addr + 2
    }

    /// Port Line Control
    ///
    /// Write: 8 bits
    fn port_line_control(&self) -> u16 {
        self.base_addr + 3
    }

    /// Port Modem Control
    ///
    /// Write: 8 bits
    fn port_modem_control(&self) -> u16 {
        self.base_addr + 4
    }

    /// Port Line Status
    ///
    /// Read: 8 bits
    fn port_line_status(&self) -> u16 {
        self.base_addr + 5
    }

    /// Initialize the UART.
    pub fn init(&self) {
        unsafe {
            x86::io::outb(self.port_interrupt_enable(), 0x00); // Disable all interrupts
            x86::io::outb(self.port_line_control(), 0x80); // Enable DLAB (set baud rate divisor)
            x86::io::outb(self.port_data(), 0x03); // Set divisor to 3 (lo byte) 38400 baud
            x86::io::outb(self.port_interrupt_enable(), 0x00); //                  (hi byte)
            x86::io::outb(self.port_line_control(), 0x03); // 8 bits, no parity, one stop bit
            x86::io::outb(self.port_fifo_control(), 0xC7); // Enable FIFO, clear them, with 14-byte threshold
            x86::io::outb(self.port_modem_control(), 0x0B); // IRQs enabled, RTS/DSR set
            x86::io::outb(self.port_interrupt_enable(), 0x01); // Enable interrupts
        }
    }

    /// Check if the UART is ready to send a byte.
    fn line_sts(&mut self) -> LineStsFlags {
        unsafe { LineStsFlags::from_bits_truncate(x86::io::inb(self.port_line_status())) }
    }

    /// Send a byte.
    pub fn send(&mut self, byte: u8) {
        match byte {
            8 | 0x7F => {
                self.send_raw(8);
                self.send_raw(b' ');
                self.send_raw(8);
            }
            byte => self.send_raw(byte),
        }
    }

    /// Send a byte on the serial port.
    pub fn send_raw(&mut self, byte: u8) {
        loop {
            if let Ok(_ok) = self.try_send_raw(byte) {
                break;
            }
            core::hint::spin_loop();
        }
    }

    /// Tries to send a byte on the serial port.
    pub fn try_send_raw(&mut self, byte: u8) -> Result<(), Uart16550Error> {
        if self.line_sts().contains(LineStsFlags::OUTPUT_EMPTY) {
            unsafe {
                x86::io::outb(self.port_data(), byte);
            }
            Ok(())
        } else {
            Err(Uart16550Error::WouldBlockError)
        }
    }

    /// Receive a byte.
    pub fn receive(&mut self) -> u8 {
        loop {
            if let Ok(byte) = self.try_receive() {
                return byte;
            }
            core::hint::spin_loop();
        }
    }

    /// Tries to receive a byte on the serial port.
    pub fn try_receive(&mut self) -> Result<u8, Uart16550Error> {
        if self.line_sts().contains(LineStsFlags::INPUT_FULL) {
            let data = unsafe { x86::io::inb(self.port_data()) };
            Ok(data)
        } else {
            Err(Uart16550Error::WouldBlockError)
        }
    }
}

impl Write for Uart16550 {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            self.send(c);
        }
        Ok(())
    }
}

lazy_static::lazy_static! {
    pub static ref SERIAL1: Mutex<Uart16550> = {
        let serial_port = unsafe { Uart16550::new(COM1) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ($crate::_serial_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::_serial_print(format_args!("\n")));
    ($($arg:tt)*) => ($crate::_serial_print(format_args!("{}\n", format_args!($($arg)*))));
}

#[doc(hidden)]
pub fn _serial_print(args: fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).unwrap();
}
