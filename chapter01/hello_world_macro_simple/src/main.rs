#![no_std]
#![no_main]

use core::{arch::global_asm, panic::PanicInfo};

mod macros;
mod uart;

global_asm!(include_str!("../asm/boot.S"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("KERNEL PANIC!");
    loop {
        core::hint::spin_loop();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    println!("Hello, world!");
    println!("Rust AArch64 kernel started.");
    println!("Decimal: {}, hexadecimal: {:#x}", 42, 42);
    print!("This is ");
    println!("a test.");
    loop {
        core::hint::spin_loop();
    }
}
