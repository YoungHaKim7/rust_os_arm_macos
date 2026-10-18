use core::fmt::{self, Write};
use core::ptr::{read_volatile, write_volatile};

// QEMU virt machine: PL011 UART.
const UART_BASE: usize = 0x0900_0000;
const UART_DR: usize = UART_BASE;
const UART_FR: usize = UART_BASE + 0x18;

// PL011 flag register: transmit FIFO full.
const UART_FR_TXFF: u32 = 1 << 5;

pub struct UartWriter;

impl UartWriter {
    pub const fn new() -> Self {
        Self
    }

    pub fn put_byte(&mut self, byte: u8) {
        unsafe {
            // Wait until the transmit FIFO has room.
            while read_volatile(UART_FR as *const u32) & UART_FR_TXFF != 0 {
                core::hint::spin_loop();
            }

            // Write the byte to the UART data register.
            write_volatile(UART_DR as *mut u32, byte as u32);
        }
    }
}

impl Write for UartWriter {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        for byte in text.bytes() {
            if byte == b'\n' {
                self.put_byte(b'\r');
            }

            self.put_byte(byte);
        }

        Ok(())
    }
}

/// Print formatted arguments to the serial console.
pub fn _print(args: fmt::Arguments<'_>) {
    let mut writer = UartWriter::new();
    let _ = writer.write_fmt(args);
}
