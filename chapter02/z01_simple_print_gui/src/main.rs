#![no_std]
#![no_main]

use core::{
    arch::global_asm,
    panic::PanicInfo,
    ptr::{read_volatile, write_volatile},
};

// QEMU virt machine: PL011 UART.
const UART_BASE: usize = 0x0900_0000;
const UART_DR: usize = UART_BASE;
const UART_FR: usize = UART_BASE + 0x18;
const UART_FR_TXFF: u32 = 1 << 5;

// Include the assembly startup code exactly once.
global_asm!(include_str!("../asm/boot.S"));

fn uart_putc(byte: u8) {
    unsafe {
        // Wait while the transmit FIFO is full.
        while read_volatile(UART_FR as *const u32) & UART_FR_TXFF != 0 {
            core::hint::spin_loop();
        }

        // Write one byte to the UART data register.
        write_volatile(UART_DR as *mut u32, byte as u32);
    }
}

fn uart_print(text: &str) {
    for &byte in text.as_bytes() {
        if byte == b'\n' {
            uart_putc(b'\r');
        }

        uart_putc(byte);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    uart_print("Hello, world!\r\n");
    uart_print("Rust AArch64 kernel started.\r\n");

    // Keep the kernel running.
    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    uart_print("Kernel panic!\r\n");

    loop {
        core::hint::spin_loop();
    }
}
