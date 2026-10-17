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
