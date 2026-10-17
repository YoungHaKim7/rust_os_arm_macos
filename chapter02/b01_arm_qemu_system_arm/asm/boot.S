.section .text.boot, "ax"
.global _start

_start:
    // Initialize the stack.
    ldr x0, =_stack_top
    mov sp, x0

    // Enter the Rust kernel.
    bl rust_main

1:
    wfe
    b 1b

