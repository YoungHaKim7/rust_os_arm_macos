ENTRY(_start)

SECTIONS
{
    /* RAM on the QEMU "virt" board starts at 0x40000000 */
    . = 0x40000000;

    .text : { KEEP(*(.text.start)) *(.text*) }
    .rodata : { *(.rodata*) }
    .data : { *(.data*) }
    .bss : { *(.bss*) *(COMMON) }
}
