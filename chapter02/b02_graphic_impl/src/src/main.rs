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

fn uart_putc(byte: u8) {
    unsafe {
        while (read_volatile(UART0_FR) & UART_FR_TXFF) != 0 {
            core::hint::spin_loop();
        }

        write_volatile(UART0_DR, byte as u32);
    }
}

fn uart_print(message: &str) {
    for byte in message.bytes() {
        uart_putc(byte);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    uart_print("Hello World!\r\n");
    uart_print("AArch64 Rust kernel booted!\r\n");

    loop {
        core::hint::spin_loop();
    }
}
