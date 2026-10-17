#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

use core::{
    arch::global_asm,
    panic::PanicInfo,
    ptr::{addr_of_mut, read_volatile, write_volatile},
};

// QEMU starts the CPU with the stack pointer (sp) = 0.
// Any function that saves a register ("push") would crash immediately.
// So the first thing after reset must be: set up a stack, then jump to Rust.
global_asm!(
    ".section .text.start",
    ".global _start",
    "_start:",
    "    mov x0, #0x40020000", // top of a stack area in RAM
    "    mov sp, x0",
    "    b  {main}",
    main = sym rust_main,
);

// ---------------------------------------------------------------------------
// UART (PL011). With "-serial stdio" this text appears in the TERMINAL.
// ---------------------------------------------------------------------------
const UART0_DR: *mut u32 = 0x0900_0000 as *mut u32;
const UART0_FR: *const u32 = 0x0900_0018 as *const u32;
const UART_FR_TXFF: u32 = 1 << 5;

// ---------------------------------------------------------------------------
// Framebuffer for the QEMU WINDOW ("ramfb" device).
// The kernel tells QEMU where its pixel buffer is, using the fw_cfg
// interface (one DMA register), then simply writes pixels into RAM.
// ---------------------------------------------------------------------------
#[repr(C)]
struct DmaAccess {
    control: u32, // bits 31..16 = fw_cfg entry, bits 3..0 = SELECT|READ/WRITE
    length: u32,
    address: u64,
}

const FW_CFG_DMA: *mut u64 = 0x0902_0010 as *mut u64; // fw_cfg DMA trigger (virt: base+16)

const DMA_BLK: *mut DmaAccess = 0x4010_0000 as *mut DmaAccess; // spare RAM
const DIR: *mut u8 = 0x4010_1000 as *mut u8; // spare RAM
const CFG: *mut u8 = 0x4010_2000 as *mut u8; // spare RAM
const FB: *mut u32 = 0x4080_0000 as *mut u32; // the pixels themselves

const W: u32 = 800;
const H: u32 = 600;

const CTL_READ: u32 = 0x02;
const CTL_WRITE: u32 = 0x10;
const CTL_SELECT: u32 = 0x08;
const FW_CFG_FILE_DIR: u16 = 0x19;

// One fw_cfg transfer. All struct fields and the register are BIG-endian.
unsafe fn fw_cfg_dma(sel: u16, write: bool, buf: *mut u8, len: u32) {
    let flags = if write { CTL_WRITE } else { CTL_READ } | CTL_SELECT;
    write_volatile(
        addr_of_mut!((*DMA_BLK).control),
        (((sel as u32) << 16) | flags).to_be(),
    );
    write_volatile(addr_of_mut!((*DMA_BLK).length), len.to_be());
    write_volatile(addr_of_mut!((*DMA_BLK).address), (buf as u64).to_be());
    write_volatile(FW_CFG_DMA, (DMA_BLK as u64).to_be());
    while read_volatile(addr_of_mut!((*DMA_BLK).control)) != 0 {}
}

// Search the fw_cfg "file directory" for the "etc/ramfb" entry, because its
// selector number and its config-struct size are not fixed between versions.
// Returns (selector, config size).
unsafe fn find_ramfb() -> (u16, u32) {
    fw_cfg_dma(FW_CFG_FILE_DIR, false, DIR, 8192);

    let count = read_volatile(DIR as *const u32).to_be() as usize;
    for i in 0..count {
        let e = DIR.add(4 + i * 64); // entry: u32 size, u16 select, u16 res, char name[56]
        let sel = read_volatile(e.add(4) as *const u16).to_be();
        let name = core::slice::from_raw_parts(e.add(8), 56);
        if name.starts_with(b"etc/ramfb\0") {
            let size = read_volatile(e as *const u32).to_be();
            return (sel, size);
        }
    }
    (0, 0)
}

// Tell QEMU: W x H pixels, 32-bit, buffer at FB. All fields big-endian.
unsafe fn ramfb_setup(sel: u16, size: u32) {
    let mut o = 0usize;

    // address of the framebuffer (u64)
    for (i, b) in (FB as u64).to_be_bytes().iter().enumerate() {
        write_volatile(CFG.add(o + i), *b);
    }
    o += 8;

    // pixel format: DRM_FORMAT_XRGB8888 ("XR24") (u32)
    for (i, b) in 0x3432_5258u32.to_be_bytes().iter().enumerate() {
        write_volatile(CFG.add(o + i), *b);
    }
    o += 4;

    // newer QEMU has an extra "flags" field here (u32, set to 0)
    if size >= 28 {
        for i in 0..4 {
            write_volatile(CFG.add(o + i), 0);
        }
        o += 4;
    }

    // width, height, stride (u32 each)
    for (i, b) in W.to_be_bytes().iter().enumerate() {
        write_volatile(CFG.add(o + i), *b);
    }
    for (i, b) in H.to_be_bytes().iter().enumerate() {
        write_volatile(CFG.add(o + 4 + i), *b);
    }
    for (i, b) in (W * 4).to_be_bytes().iter().enumerate() {
        write_volatile(CFG.add(o + 8 + i), *b);
    }
    o += 12;

    fw_cfg_dma(sel, true, CFG, o as u32);
}

// 8x8 font, one byte per row, leftmost pixel = bit 7.
// Lowercase letters with a real x-height (e, o, w, r start lower down).
fn glyph(c: u8) -> [u8; 8] {
    match c {
        b'h' => [0x40, 0x40, 0x40, 0x78, 0x44, 0x44, 0x44, 0x00],
        b'e' => [0x00, 0x00, 0x38, 0x44, 0x7C, 0x40, 0x38, 0x00],
        b'l' => [0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x78, 0x00],
        b'o' => [0x00, 0x00, 0x38, 0x44, 0x44, 0x44, 0x38, 0x00],
        b'w' => [0x00, 0x00, 0x81, 0x81, 0x89, 0x89, 0x4A, 0x00],
        b'r' => [0x00, 0x00, 0x40, 0x78, 0x44, 0x40, 0x00, 0x00],
        b'd' => [0x04, 0x04, 0x7C, 0x84, 0x84, 0x7C, 0x00, 0x00],
        b' ' => [0x00; 8],
        _ => [0x00; 8],
    }
}

// Draw a string into the framebuffer, every font pixel becomes scale x scale.
unsafe fn draw_text(s: &[u8], x0: usize, y0: usize, scale: usize) {
    let mut x = x0;
    for &c in s {
        let g = glyph(c);
        for row in 0..8 {
            for col in 0..8 {
                if g[row] & (0x80 >> col) != 0 {
                    for sy in 0..scale {
                        for sx in 0..scale {
                            let p = FB.add((y0 + row * scale + sy) * W as usize + x + col * scale + sx);
                            write_volatile(p, 0x00FF_FFFF); // white pixel
                        }
                    }
                }
            }
        }
        x += 8 * scale;
    }
}

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

    unsafe {
        let (sel, size) = find_ramfb();
        if sel == 0 {
            uart_print("etc/ramfb not found (run QEMU with -device ramfb)\r\n");
        } else {
            ramfb_setup(sel, size);
            draw_text(b"hello world", 64, 64, 4);
        }
    }

    loop {
        core::hint::spin_loop();
    }
}
