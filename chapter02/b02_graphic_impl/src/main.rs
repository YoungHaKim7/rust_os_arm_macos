#![no_std]
#![no_main]

use core::{
    arch::global_asm,
    panic::PanicInfo,
    ptr::{addr_of, addr_of_mut, read_volatile, write_volatile},
    sync::atomic::{Ordering, fence},
};

global_asm!(include_str!("../asm/boot.S"));

// PL011 UART on QEMU ARM virt.
const UART_DR: *mut u32 = 0x0900_0000 as *mut u32;
const UART_FR: *const u32 = 0x0900_0018 as *const u32;
const UART_TXFF: u32 = 1 << 5;

// QEMU fw_cfg MMIO registers on the ARM virt machine.
const FW_CFG_BASE: usize = 0x0902_0000;
const FW_CFG_DATA: *const u8 = FW_CFG_BASE as *const u8;
const FW_CFG_SELECTOR: *mut u16 = (FW_CFG_BASE + 8) as *mut u16;
const FW_CFG_DMA_ADDR: *mut u64 = (FW_CFG_BASE + 0x10) as *mut u64;

const FW_CFG_FILE_DIR: u16 = 0x0019;
const FW_CFG_DMA_SELECT: u32 = 1 << 3;
const FW_CFG_DMA_WRITE: u32 = 1 << 4;
const FW_CFG_DMA_ERROR: u32 = 1;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const STRIDE: u32 = (WIDTH * 4) as u32;

// DRM_FORMAT_XRGB8888: fourcc('X', 'R', '2', '4').
const XRGB8888: u32 = u32::from_le_bytes(*b"XR24");

#[repr(C, align(4096))]
struct Framebuffer([u32; WIDTH * HEIGHT]);

static mut FRAMEBUFFER: Framebuffer = Framebuffer([0; WIDTH * HEIGHT]);

#[repr(C)]
struct RamfbConfig {
    address: u64,
    fourcc: u32,
    flags: u32,
    width: u32,
    height: u32,
    stride: u32,
}

#[repr(C, align(16))]
struct DmaAccess {
    control: u32,
    length: u32,
    address: u64,
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

fn uart_putc(byte: u8) {
    unsafe {
        while (read_volatile(UART_FR) & UART_TXFF) != 0 {
            core::hint::spin_loop();
        }

        write_volatile(UART_DR, byte as u32);
    }
}

fn uart_print(s: &str) {
    for byte in s.bytes() {
        uart_putc(byte);
    }
}

unsafe fn fw_cfg_select(index: u16) {
    unsafe {
        write_volatile(FW_CFG_SELECTOR, index.to_be());
    }
}

unsafe fn fw_cfg_read_byte() -> u8 {
    unsafe { read_volatile(FW_CFG_DATA) }
}

// Locate the selector for QEMU's "etc/ramfb" item.
unsafe fn find_ramfb_selector() -> Option<u16> {
    unsafe {
        // Test whether the fw_cfg device responds.
        uart_print("[fw_cfg] Selecting signature...\r\n");
        fw_cfg_select(0x0000);

        uart_print("[fw_cfg] Reading signature...\r\n");

        let mut signature = [0u8; 4];

        for byte in &mut signature {
            *byte = fw_cfg_read_byte();
        }

        uart_print("[fw_cfg] Signature: ");

        for byte in signature {
            uart_putc(byte);
        }

        uart_print("\r\n");

        if signature != *b"QEMU" {
            uart_print("[fw_cfg] ERROR: invalid signature\r\n");
            return None;
        }

        // Read the file directory header.
        uart_print("[fw_cfg] Selecting file directory...\r\n");
        fw_cfg_select(FW_CFG_FILE_DIR);

        let mut header = [0u8; 4];

        for byte in &mut header {
            *byte = fw_cfg_read_byte();
        }

        let count = u32::from_be_bytes(header);

        uart_print("[fw_cfg] Directory entries: 0x");
        uart_hex(count);
        uart_print("\r\n");

        if count > 4096 {
            uart_print("[fw_cfg] ERROR: unreasonable entry count\r\n");
            return None;
        }

        for i in 0..count {
            uart_print("[fw_cfg] Entry ");
            uart_hex(i);
            uart_print("\r\n");

            let mut entry = [0u8; 64];

            // Test the first byte without printing beforehand.
            entry[0] = fw_cfg_read_byte();

            uart_print("[fw_cfg] First byte read: 0x");
            uart_hex(entry[0] as u32);
            uart_print("\r\n");

            // Read the remaining 63 bytes.
            for byte in &mut entry[1..] {
                *byte = fw_cfg_read_byte();
            }

            uart_print("[fw_cfg] Entry complete\r\n");

            if &entry[8..17] == b"etc/ramfb" {
                let selector = u16::from_be_bytes([entry[4], entry[5]]);
                uart_print("[fw_cfg] Found etc/ramfb\r\n");
                return Some(selector);
            }
        }

        uart_print("[fw_cfg] ERROR: etc/ramfb not found\r\n");
        None
    }
}

// Submit a DMA write to the selected fw_cfg item.
unsafe fn fw_cfg_dma_write(selector: u16, source: *const u8, length: u32) -> bool {
    let mut dma = DmaAccess {
        control: 0,
        length: length.to_be(),
        address: (source as u64).to_be(),
    };

    let dma_ptr = addr_of_mut!(dma);

    unsafe {
        write_volatile(
            addr_of_mut!((*dma_ptr).control),
            (((selector as u32) << 16) | FW_CFG_DMA_SELECT | FW_CFG_DMA_WRITE).to_be(),
        );

        fence(Ordering::SeqCst);

        write_volatile(FW_CFG_DMA_ADDR, (dma_ptr as u64).to_be());

        // QEMU completes the transfer synchronously today.
        // The limit prevents an infinite wait if it fails.
        for _ in 0..1_000_000 {
            fence(Ordering::SeqCst);

            let control = u32::from_be(read_volatile(addr_of!((*dma_ptr).control)));

            if control == 0 {
                return true;
            }

            if (control & FW_CFG_DMA_ERROR) != 0 {
                return false;
            }

            core::hint::spin_loop();
        }
    }

    false
}

fn glyph(c: u8) -> [u8; 7] {
    match c {
        b'H' => [
            0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        b'e' => [0, 0b01110, 0b10001, 0b11111, 0b10000, 0b01110, 0],
        b'l' => [0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110, 0],
        b'o' => [0, 0b01110, 0b10001, 0b10001, 0b10001, 0b01110, 0],
        b'W' => [
            0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b10101, 0b01010,
        ],
        b'r' => [0, 0b10110, 0b11001, 0b10000, 0b10000, 0b10000, 0],
        b'd' => [0b00001, 0b00001, 0b01111, 0b10001, 0b10001, 0b01111, 0],
        b'!' => [0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0, 0b00100],
        b' ' => [0; 7],
        _ => [0; 7],
    }
}

fn pixel(x: usize, y: usize, color: u32) {
    if x < WIDTH && y < HEIGHT {
        unsafe {
            let fb = addr_of_mut!(FRAMEBUFFER.0).cast::<u32>();
            write_volatile(fb.add(y * WIDTH + x), color);
        }
    }
}

fn draw_text(text: &[u8], x: usize, y: usize, scale: usize, color: u32) {
    let mut cursor_x = x;

    for &character in text {
        let rows = glyph(character);

        for (row, bits) in rows.iter().enumerate() {
            for col in 0..5 {
                if bits & (1 << (4 - col)) != 0 {
                    for dy in 0..scale {
                        for dx in 0..scale {
                            pixel(cursor_x + col * scale + dx, y + row * scale + dy, color);
                        }
                    }
                }
            }
        }

        cursor_x += 6 * scale;
    }
}

fn draw_screen() {
    // Dark blue-gray background.
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            pixel(x, y, 0x0010_1828);
        }
    }

    // Large white bitmap text.
    draw_text(b"Hello World!", 80, 100, 8, 0x00FF_FFFF);

    // Green subtitle using the same bitmap font.
    // Only glyphs defined above will be visible.
    draw_text(b"Rust", 80, 190, 4, 0x0000_FF80);
}

fn configure_framebuffer() -> bool {
    uart_print("[ramfb] Searching fw_cfg directory...\r\n");

    let selector = unsafe { find_ramfb_selector() };

    let Some(selector) = selector else {
        uart_print("[ramfb] ERROR: etc/ramfb not found\r\n");
        return false;
    };

    uart_print("[ramfb] Found etc/ramfb\r\n");

    let framebuffer_address = addr_of_mut!(FRAMEBUFFER) as *mut u32 as u64;

    uart_print("[ramfb] Preparing configuration...\r\n");

    let config = RamfbConfig {
        address: framebuffer_address.to_be(),
        fourcc: XRGB8888.to_be(),
        flags: 0u32.to_be(),
        width: (WIDTH as u32).to_be(),
        height: (HEIGHT as u32).to_be(),
        stride: STRIDE.to_be(),
    };

    uart_print("[ramfb] Starting DMA write...\r\n");

    let success = unsafe {
        fw_cfg_dma_write(
            selector,
            &config as *const RamfbConfig as *const u8,
            core::mem::size_of::<RamfbConfig>() as u32,
        )
    };

    uart_print("[ramfb] DMA write returned\r\n");

    if !success {
        uart_print("[ramfb] ERROR: DMA configuration failed\r\n");
        return false;
    }

    uart_print("[ramfb] Configuration successful\r\n");
    true
}

fn uart_hex(value: u32) {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";

    for shift in [28, 24, 20, 16, 12, 8, 4, 0] {
        let digit = ((value >> shift) & 0xF) as usize;
        uart_putc(HEX[digit]);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    uart_print("Rust AArch64 kernel started\r\n");

    draw_screen();

    uart_print("Framebuffer pixels drawn\r\n");

    if configure_framebuffer() {
        uart_print("ramfb configured\r\n");
    }

    loop {
        core::hint::spin_loop();
    }
}
