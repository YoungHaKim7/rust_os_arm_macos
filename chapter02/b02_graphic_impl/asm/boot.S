.section .text.boot, "ax"
.global _start
.type _start, %function

_start:
    msr     daifset, #0xf

    // Enable FP/SIMD access at EL1.
    mrs     x0, cpacr_el1
    mov     x1, #0x300000
    orr     x0, x0, x1
    msr     cpacr_el1, x0
    isb

    ldr     x0, =_stack_top
    mov     sp, x0

    bl      rust_main

1:
    wfe
    b       1b

.size _start, . - _start
