- Your Rust code is a reasonable starting point for a bare-metal UART hello-world kernel, but you're right about the missing piece: you need a way to load and execute it.

The important distinction is that your code is not a complete bootloader. It is a small freestanding program intended to run after something has initialized the machine and transferred control to `_start`.

For your example, I would start with QEMU's ARM `virt` machine and its built-in firmware, rather than GRUB. GRUB is primarily associated with x86 PC booting; your UART address `0x0900_0000` matches the PL011 UART commonly provided by QEMU's ARM `virt` machine.

There are three pieces to make this work:

1\. QEMU firmware

Initializes the virtual machine and loads or launches the next stage.

2\. Rust kernel

An ELF executable with an entry point, linker layout, and suitable target architecture.

3\. UART output

Writes bytes to the emulated serial device so QEMU can display “Hello World!” in the terminal.

There are also two specific problems in the supplied code that should be corrected before testing it:

- `UART0_FR` should point to the flag register at `0x0900_0018`, not `0x0900_0029`. The flag register is a 32-bit register, and `0x0900_0029` is not its correct address.
- The program must be compiled for the correct CPU architecture. The code's register addresses suggest AArch64 or AArch32 on QEMU ARM `virt`, not a normal macOS or Linux host executable.

## 1. Corrected UART addresses

The PL011 register offsets are documented in the QEMU implementation.&#x20;

[image](https://www.google.com/s2/favicons?domain=https://qemu.googlesource.com\&sz=32)

Git at Google

+1



| Register | Address        | Purpose               |
| -------- | -------------- | --------------------- |
| `UARTDR` | `0x0900_0000`  | Transmit/receive data |
| `UARTFR` | `0x0900_0018`  | UART status flags     |
| `TXFF`   | Bit 5 (`0x20`) | Transmit FIFO full    |

The address calculation is:

```
UART base       = 0x09000000
UARTDR offset   = 0x00000000
UARTFR offset   = 0x00000018

UARTDR address  = 0x09000000
UARTFR address  = 0x09000018
```

Your original `UART0_FR` address is incorrect, but that is a fixable register-address bug, not a reason to reject the entire approach.

## 2. Build a minimal Rust kernel that boots in QEMU

This version includes a tiny assembly entry point to initialize the stack before calling Rust. QEMU can load the resulting bare-metal ELF directly, so you don't need to install GRUB or write a separate disk bootloader for this first milestone.&#x20;

[image](https://www.google.com/s2/favicons?domain=https://wiki.osdev.org\&sz=32)

OSDev Wiki

+1



The boot flow will be:

QEMU `virt` / AArch64

QEMU loads `kernel.elf`

`_start`: initialize stack

`rust_main`: write to UART

Hello World!

### Project layout

Create the following files:

```bash
rust-kernel/
├── Cargo.toml
├── linker.ld
├── boot.S
├── .cargo/
│   └── config.toml
└── src/
    └── main.rs
```

### `Cargo.toml`

```toml

[package]
name = "rust-kernel"
version = "0.1.0"
edition = "2024"

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"

```

### `.cargo/config.toml`

```toml
[build]
target = "aarch64-unknown-none"

[target.aarch64-unknown-none]
rustflags = [
    "-C", "link-arg=-Tlinker.ld",
]
```

This selects Rust's bare-metal AArch64 target instead of compiling a normal macOS or Linux application.

### `boot.S`

```asm
.section .text.boot, "ax"
.global _start
.type _start, %function

_start:
    // Initialize the stack pointer.
    ldr x0, =_stack_top
    mov sp, x0

    // Call the Rust kernel entry point.
    bl rust_main

1:
    wfe
    b 1b

.size _start, . - _start

```

This is the small assembly portion that Rust alone cannot safely replace without some startup assumptions. It ensures a stack exists before entering an ordinary Rust function.

### `linker.ld`

```ld

ENTRY(_start)

SECTIONS
{
    . = 0x40080000;

    .text : ALIGN(4096) {
        KEEP(*(.text.boot))
        *(.text .text.*)
    }

    .rodata : ALIGN(4096) {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(4096) {
        *(.data .data.*)
    }

    .bss (NOLOAD) : ALIGN(4096) {
        *(.bss .bss.*)
        *(COMMON)

        . = ALIGN(16);
        . += 16384;
        _stack_top = .;
    }

    /DISCARD/ : {
        *(.eh_frame .eh_frame.*)
    }
}

```

The linker script determines where the kernel is placed in memory and tells the linker that `_start` is the entry point. The RAM address is chosen for this QEMU setup; it is not a universal address for every ARM board.

### `src/main.rs`

```rs

#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;
use core::ptr::{read_volatile, write_volatile};

global_asm!(include_str!("../boot.S"));

const UART0_DR: *mut u8 = 0x0900_0000 as *mut u8;
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
            while (read_volatile(UART0_FR) & UART_FR_TXFF) != 0 {
                core::hint::spin_loop();
            }

            write_volatile(UART0_DR, byte);
        }
    }

    loop {
        core::hint::spin_loop();
    }
}

```

Notice the distinction between `_start` and `rust_main`:

- `_start` is the assembly entry point.
- `_stack_top` is a linker-defined address used to initialize the stack.
- `rust_main` is the Rust function that prints the message.
- `read_volatile` and `write_volatile` are used because the UART registers are memory-mapped hardware registers.

The kernel doesn't use `std`, an operating system, or a normal process entry point.

## 3. Install the tools, compile, and boot

On macOS with Apple Silicon, install QEMU and the Rust target:

```bash
brew install qemu

rustup target add aarch64-unknown-none
```

The target is officially supported for freestanding AArch64 development and includes `core` without requiring an operating system.&#x20;

[image](https://www.google.com/s2/favicons?domain=https://doc.rust-lang.org\&sz=32)

The rustc book



Build the kernel from the project root:

```bash
cargo build
```

Then run it:

```bash
qemu-system-aarch64 \
    -machine virt \
    -cpu cortex-a57 \
    -m 256M \
    -nographic \
    -kernel target/aarch64-unknown-none/debug/rust-kernel
```

Expected output:

```
Hello World!
```

To exit QEMU, press `Ctrl-A`, then `X`.

If compilation fails, check the actual linker output first. If the kernel boots but prints nothing, inspect the ELF entry point and disassembly, verify the UART address, and check the CPU's initial exception level and boot assumptions.

## 4. Do you need GRUB or your own bootloader?

- It depends on your goal.

| Goal                                    | Recommended approach                                           |
| --------------------------------------- | -------------------------------------------------------------- |
| Learn bare-metal Rust                   | QEMU direct ELF loading                                        |
| Learn a real boot protocol              | UEFI or a dedicated bootloader                                 |
| Build an x86-64 OS                      | GRUB with a supported Multiboot protocol                       |
| Build an ARM64 OS                       | AArch64 startup assembly, firmware or a suitable bootloader    |
| Boot from a disk image on real hardware | Firmware plus a bootloader and a properly formatted boot image |

- One important distinction: QEMU's `-kernel` option loads your ELF for you; it is not a portable bootloader that you can put on a physical machine. The startup code above also assumes QEMU's AArch64 execution environment and does not yet initialize every piece of processor state a production kernel would need.

For your next step, I recommend getting this minimal kernel running first, then adding a proper bootloader that loads the ELF and transfers control to `_start`. That way, when something goes wrong, you can distinguish a boot problem from a Rust compiler problem or a UART driver problem.

