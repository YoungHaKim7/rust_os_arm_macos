#![no_std]
#![no_main]

use core::arch::global_asm;
use core::mem::size_of;
use core::panic::PanicInfo;
use core::ptr::{addr_of, addr_of_mut, read_volatile, write_volatile};

global_asm!(include_str!("../asm/boot.S"));

// ============================================================
// Kernel configuration
// ============================================================

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const BYTES_PER_PIXEL: usize = 4;
const STRIDE: usize = WIDTH * BYTES_PER_PIXEL;

// QEMU virt machine PL011 UART.
const UART_BASE: usize = 0x0900_0000;
const UART_DR: usize = UART_BASE;
const UART_FR: usize = UART_BASE + 0x18;
const UART_FR_TXFF: u32 = 1 << 5;

// QEMU fw_cfg MMIO interface.
const FW_CFG_BASE: usize = 0x0902_0000;
const FW_CFG_DATA: usize = FW_CFG_BASE;
const FW_CFG_SELECTOR: usize = FW_CFG_BASE + 0x08;
const FW_CFG_DMA: usize = FW_CFG_BASE + 0x10;

// fw_cfg selector values.
const FW_CFG_SIGNATURE: u16 = 0x0000;
const FW_CFG_FILE_DIR: u16 = 0x0019;

// fw_cfg DMA control bits.
const FW_CFG_DMA_CTL_ERROR: u32 = 0x01;
const FW_CFG_DMA_CTL_READ: u32 = 0x02;
const FW_CFG_DMA_CTL_SKIP: u32 = 0x04;
const FW_CFG_DMA_CTL_SELECT: u32 = 0x08;
const FW_CFG_DMA_CTL_WRITE: u32 = 0x10;

// DRM_FORMAT_XRGB8888 = fourcc_code('X', 'R', '2', '4').
const DRM_FORMAT_XRGB8888: u32 = u32::from_le_bytes(*b"XR24");

// Colors are represented as 0x00RRGGBB.
const COLOR_BACKGROUND: u32 = 0x00101830;
const COLOR_PANEL: u32 = 0x00202D48;
const COLOR_TEXT: u32 = 0x00FFFFFF;
const COLOR_ACCENT: u32 = 0x0000C8FF;

// ============================================================
// Framebuffer
// ============================================================

// The framebuffer resides in ordinary guest RAM.
// QEMU ramfb will display the pixels stored here.
static mut FRAMEBUFFER: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

// ============================================================
// fw_cfg DMA structures
// ============================================================

// The DMA descriptor is 16 bytes, with all fields in big-endian
// byte order. Its address must be 8-byte aligned.
#[repr(C, align(8))]
struct FwCfgDmaAccess {
    control: u32,
    length: u32,
    address: u64,
}

// QEMU ramfb configuration is 28 bytes.
// All fields are stored in big-endian byte order.
#[repr(C)]
struct RamfbConfig {
    addr: u64,
    fourcc: u32,
    flags: u32,
    width: u32,
    height: u32,
    stride: u32,
}

// ============================================================
// Kernel entry point
// ============================================================

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    uart_print("\r\nRust AArch64 kernel started\r\n");

    uart_print("Drawing framebuffer pixels...\r\n");
    draw_background();
    draw_text("Hello World!", 220, 260, 5, COLOR_TEXT);
    draw_text("Rust AArch64", 250, 320, 3, COLOR_ACCENT);
    uart_print("Framebuffer pixels drawn\r\n");

    uart_print("[ramfb] Searching fw_cfg directory...\r\n");

    let selector = match find_ramfb_selector() {
        Some(value) => value,
        None => {
            uart_print("[ramfb] ERROR: etc/ramfb not found\r\n");
            halt();
        }
    };

    uart_print("[ramfb] Found etc/ramfb selector: 0x");
    uart_hex(selector as u32);
    uart_print("\r\n");

    let framebuffer_address = addr_of!(FRAMEBUFFER) as *const u32 as u64;

    uart_print("[ramfb] Framebuffer address: 0x");
    uart_hex((framebuffer_address >> 32) as u32);
    uart_hex(framebuffer_address as u32);
    uart_print("\r\n");

    uart_print("[ramfb] Preparing configuration...\r\n");

    let config = RamfbConfig {
        addr: framebuffer_address.to_be(),
        fourcc: DRM_FORMAT_XRGB8888.to_be(),
        flags: 0u32.to_be(),
        width: (WIDTH as u32).to_be(),
        height: (HEIGHT as u32).to_be(),
        stride: (STRIDE as u32).to_be(),
    };

    uart_print("[ramfb] Starting DMA write...\r\n");

    let status = unsafe {
        fw_cfg_dma_transfer(
            selector,
            FW_CFG_DMA_CTL_SELECT | FW_CFG_DMA_CTL_WRITE,
            &config as *const RamfbConfig as u64,
            size_of::<RamfbConfig>() as u32,
        )
    };

    uart_print("[ramfb] DMA status: 0x");
    uart_hex(status);
    uart_print("\r\n");

    if status & FW_CFG_DMA_CTL_ERROR != 0 {
        uart_print("[ramfb] ERROR: DMA reported an error\r\n");
        halt();
    }

    if status != 0 {
        uart_print("[ramfb] ERROR: Unexpected DMA status\r\n");
        halt();
    }

    uart_print("[ramfb] DMA configuration successful\r\n");
    uart_print("[ramfb] Hello World! should be visible in QEMU\r\n");

    // Keep the kernel alive so the display remains available.
    halt();
}

// ============================================================
// PL011 UART
// ============================================================

fn uart_putc(byte: u8) {
    unsafe {
        // Wait until the UART transmit FIFO has space.
        while read_volatile(UART_FR as *const u32) & UART_FR_TXFF != 0 {
            core::hint::spin_loop();
        }

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

fn uart_hex(value: u32) {
    for shift in (0u32..8).rev() {
        let nibble = ((value >> (shift * 4)) & 0xF) as u8;

        let character = if nibble < 10 {
            b'0' + nibble
        } else {
            b'A' + nibble - 10
        };

        uart_putc(character);
    }
}

// ============================================================
// QEMU fw_cfg access
// ============================================================

unsafe fn fw_cfg_select(selector: u16) {
    unsafe {
        // The selector register expects big-endian byte order.
        write_volatile(FW_CFG_SELECTOR as *mut u16, selector.to_be());
    }
}

unsafe fn fw_cfg_read_byte() -> u8 {
    unsafe { read_volatile(FW_CFG_DATA as *const u8) }
}

unsafe fn fw_cfg_read_u32() -> u32 {
    let mut bytes = [0u8; 4];

    for byte in &mut bytes {
        *byte = unsafe { fw_cfg_read_byte() };
    }

    u32::from_be_bytes(bytes)
}

fn find_ramfb_selector() -> Option<u16> {
    uart_print("[fw_cfg] Selecting signature...\r\n");

    unsafe {
        fw_cfg_select(FW_CFG_SIGNATURE);
    }

    uart_print("[fw_cfg] Reading signature...\r\n");

    let mut signature = [0u8; 4];

    for byte in &mut signature {
        *byte = unsafe { fw_cfg_read_byte() };
    }

    uart_print("[fw_cfg] Signature: ");

    for &byte in &signature {
        uart_putc(byte);
    }

    uart_print("\r\n");

    if &signature != b"QEMU" {
        uart_print("[fw_cfg] ERROR: Invalid QEMU signature\r\n");
        return None;
    }

    uart_print("[fw_cfg] Selecting file directory...\r\n");

    unsafe {
        fw_cfg_select(FW_CFG_FILE_DIR);
    }

    // The directory begins with a big-endian u32 entry count.
    let count = unsafe { fw_cfg_read_u32() };

    uart_print("[fw_cfg] Directory entries: 0x");
    uart_hex(count);
    uart_print("\r\n");

    // Each fw_cfg directory entry is exactly 64 bytes:
    //
    // Offset  Size  Field
    // 0       4     File size
    // 4       2     Selector
    // 6       2     Reserved
    // 8       56    NUL-terminated filename
    //
    // Limit the count to prevent a malformed directory from
    // causing an unbounded scan.
    if count > 4096 {
        uart_print("[fw_cfg] ERROR: Invalid directory count\r\n");
        return None;
    }

    for index in 0..count {
        uart_print("[fw_cfg] Entry ");
        uart_hex(index);
        uart_print("\r\n");

        let mut entry = [0u8; 64];

        for byte in &mut entry {
            *byte = unsafe { fw_cfg_read_byte() };
        }

        let selector = u16::from_be_bytes([entry[4], entry[5]]);
        let name_bytes = &entry[8..64];

        let name_len = name_bytes
            .iter()
            .position(|&byte| byte == 0)
            .unwrap_or(name_bytes.len());

        let name = &name_bytes[..name_len];

        if name == b"etc/ramfb" {
            uart_print("[fw_cfg] Found etc/ramfb\r\n");
            return Some(selector);
        }
    }

    None
}

// ============================================================
// QEMU fw_cfg DMA
// ============================================================

unsafe fn fw_cfg_dma_transfer(
    selector: u16,
    operation: u32,
    buffer_address: u64,
    length: u32,
) -> u32 {
    let control = ((selector as u32) << 16) | operation;

    let mut descriptor = FwCfgDmaAccess {
        control: control.to_be(),
        length: length.to_be(),
        address: buffer_address.to_be(),
    };

    let descriptor_address = addr_of_mut!(descriptor) as u64;

    // Writing the descriptor's big-endian address to the DMA
    // register starts the operation.
    unsafe {
        write_volatile(FW_CFG_DMA as *mut u64, descriptor_address.to_be());
    }

    // QEMU completes the DMA operation and writes the status
    // back into the descriptor's control field.
    //
    // Volatile access is important because the device updates
    // memory independently of normal Rust code.
    let raw_status = unsafe { read_volatile(addr_of!(descriptor.control)) };

    u32::from_be(raw_status)
}

// ============================================================
// Drawing
// ============================================================

fn draw_background() {
    let framebuffer = addr_of_mut!(FRAMEBUFFER) as *mut u32;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let color = if (x / 40 + y / 40) % 2 == 0 {
                COLOR_BACKGROUND
            } else {
                COLOR_PANEL
            };

            let index = y * WIDTH + x;

            unsafe {
                write_volatile(framebuffer.add(index), color);
            }
        }
    }

    // A simple horizontal accent line.
    for x in 150..650 {
        draw_pixel(x, 230, COLOR_ACCENT);
        draw_pixel(x, 231, COLOR_ACCENT);
    }
}

fn draw_pixel(x: usize, y: usize, color: u32) {
    if x >= WIDTH || y >= HEIGHT {
        return;
    }

    let framebuffer = addr_of_mut!(FRAMEBUFFER) as *mut u32;

    let index = y * WIDTH + x;

    unsafe {
        write_volatile(framebuffer.add(index), color);
    }
}

// Each glyph is a 5x7 bitmap. The most significant of the
// five low bits represents the leftmost pixel.
fn glyph(character: u8) -> [u8; 7] {
    match character {
        b'H' => [
            0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],

        b'e' => [
            0b00000, 0b01110, 0b10001, 0b11111, 0b10000, 0b01110, 0b00000,
        ],

        b'l' => [
            0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],

        b'o' => [
            0b00000, 0b01110, 0b10001, 0b10001, 0b10001, 0b01110, 0b00000,
        ],

        b'W' => [
            0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b10101, 0b01010,
        ],

        b'r' => [
            0b00000, 0b10110, 0b11001, 0b10000, 0b10000, 0b10000, 0b10000,
        ],

        b'd' => [
            0b00001, 0b00001, 0b01101, 0b10011, 0b10001, 0b10011, 0b01101,
        ],

        b'!' => [
            0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00000, 0b00100,
        ],

        b'A' => [
            0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],

        b'R' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
        ],

        b'u' => [
            0b00000, 0b10001, 0b10001, 0b10001, 0b10011, 0b01101, 0b00000,
        ],

        b's' => [
            0b00000, 0b01111, 0b10000, 0b01110, 0b00001, 0b11110, 0b00000,
        ],

        b't' => [
            0b00100, 0b01111, 0b00100, 0b00100, 0b00100, 0b00101, 0b00010,
        ],

        b' ' => [0; 7],

        _ => [0; 7],
    }
}

fn draw_text(text: &str, x: usize, y: usize, scale: usize, color: u32) {
    let mut cursor_x = x;

    for character in text.bytes() {
        let bitmap = glyph(character);

        for (row, bits) in bitmap.iter().copied().enumerate() {
            for column in 0..5 {
                let mask = 1u8 << (4 - column);

                if bits & mask != 0 {
                    for dy in 0..scale {
                        for dx in 0..scale {
                            draw_pixel(cursor_x + column * scale + dx, y + row * scale + dy, color);
                        }
                    }
                }
            }
        }

        cursor_x += 6 * scale;
    }
}

// ============================================================
// Panic and halt
// ============================================================

fn halt() -> ! {
    loop {
        unsafe {
            core::arch::asm!("wfe", options(nomem, nostack));
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    uart_print("\r\nKERNEL PANIC\r\n");
    halt()
}
