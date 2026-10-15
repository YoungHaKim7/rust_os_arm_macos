#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let uart = 0x9000_0000 as *mut u8;
    let hello = b"Hello World!\n";

    for &byte in hello {
        unsafe {
            *uart = byte;
        }
    }

    loop {}
}
