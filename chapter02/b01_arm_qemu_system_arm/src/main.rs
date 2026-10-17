#![no_std]
#![no_main]

use core::{
    panic::PanicInfo,
    ptr::{read_volatile, write_volatile},
};

const UART0_DR: *mut u32 = 0x0900_0000 as *mut u32;
const UART0_FR: *const u32 = 0x0900_0018 as *const u32;

const UART_FR_TXFF: u32 = 1 << 5;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    let message = b"Hello World!\r\n";

    for &byte in message {
        unsafe {
            // Wait until the transmit FIFO is not full.
            while (read_volatile(UART0_FR) & UART_FR_TXFF) != 0 {
                core::hint::spin_loop();
            }

            // Write one character to the UART data register.
            write_volatile(UART0_DR, byte as u32);
        }
    }

    // Keep the kernel running.
    loop {
        core::hint::spin_loop();
    }
}
