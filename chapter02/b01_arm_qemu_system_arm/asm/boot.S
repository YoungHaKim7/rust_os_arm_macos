.section .text.boot, "ax"
.global _start
.type _start, %function

_start:
    msr daifset, #0xf

    ldr x0, =_stack_top
    mov sp, x0

    bl rust_main

1:
    wfe
    b 1b

.size _start, . - _start

