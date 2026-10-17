# justfile

```bash
just qemu
```

The error `symbol '_start' is already defined` means your project contains two definitions of `_start`. If you want the kernel to begin execution at `_start` in `asm/boot.S`, keep that assembly definition and remove or rename the duplicate definition elsewhere.

## 1. Modify `/asm/boot.S`

Use this as your starting point:

```asm
.section .text.boot, "ax"
.global _start
.type _start, %function

_start:
    // 1. Initialize the stack pointer.
    ldr     x0, =_stack_top
    mov     sp, x0

    // 2. Enable FP/SIMD for Rust code, if needed.
    mrs     x0, cpacr_el1
    mov     x1, #0x300000
    orr     x0, x0, x1
    msr     cpacr_el1, x0
    isb

    // 3. Call the Rust entry point.
    bl      rust_main

    // 4. Halt if rust_main returns.
1:
    wfe
    b       1b

.size _start, . - _start
```

This assumes your linker script defines `_stack_top` and your Rust code defines `rust_main`.

Important: Your original code has `bl rust_main` commented out, so even without the duplicate-symbol error, it would never enter Rust. The version above fixes that.

## 2. Find the duplicate `_start`

From your project root, run:

```bash
rg -n '(_start|global_asm|no_mangle|export_name)' .
```

Check especially these common causes:

- `src/main.rs` contains another `fn _start()`.
- Another assembly file defines `_start`.
- A `global_asm!` block embeds assembly that defines `_start`, while `boot.S` also defines it.
- A build script or included assembly file adds a second definition.

If `src/main.rs` currently defines `_start`, remove that function and retain the Rust kernel entry point, for example:

```rs
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
```

If your Rust edition or toolchain expects the older attribute syntax, use `#[no_mangle]` instead of `#[unsafe(no_mangle)]`.

## 3. Make sure `boot.S` is included only once

If you include the assembly from Rust, use one inclusion:

```rs
core::arch::global_asm!(include_str!("../asm/boot.S"));
```

Do not also compile `boot.S` separately through `build.rs` or another build configuration.

## 4. Check the linker script

Your `/asm/linker.ld` should place the boot section at the kernel's load address and define the stack:

```ld
ENTRY(_start)

SECTIONS
{
    . = 0x40080000;

    .text : {
        KEEP(*(.text.boot))
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data .data.*)
    }

    .bss : {
        . = ALIGN(16);
        *(.bss .bss.*)
        *(COMMON)
    }

    . = ALIGN(16);
    . += 0x10000;
    _stack_top = .;
}
```

This is a minimal example; preserve any extra sections or memory layout your graphics kernel requires.

## 5. Build and run

```bash
cargo clean
cargo build --target aarch64-unknown-none
just qemu
```

The intended startup sequence is:

```
QEMU loads kernel
       |
       v
     _start          <- asm/boot.S
       |
       +-- Initialize SP
       |
       +-- Enable FP/SIMD
       |
       v
    rust_main        <- src/main.rs
       |
       v
  Kernel initialization
```

- The key fix is to have exactly one `_start` symbol, defined in `boot.S`, and call `rust_main` from it.
