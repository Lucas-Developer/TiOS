KERNEL_VMA = 0xffff800000000000;
KERNEL_LMA = 1M;

ENTRY(start)

SECTIONS {
    . = KERNEL_LMA;

    .boot : {
        KEEP(*(.boot))
        . = ALIGN(4K);
    }

    .bootstrap : {
        *(.bootstrap)
    }

    .bss.bootstrap : 
    {
        *(.bss.bootstrap)
    }

    .rodata.bootstrap :
    {
        *(.rodata.bootstrap)
    }

    . += KERNEL_VMA;

    .rodata : AT(ADDR(.rodata) - KERNEL_VMA)
    {
        *(EXCLUDE_FILE(*src/arch/boot.o) .rodata .rodata.*)
        . = ALIGN(4K);
    }

    .text : AT(ADDR(.text) - KERNEL_VMA)
    {
        *(.text .text.*)
        . = ALIGN(4K);
    }

    .data : AT(ADDR(.data) - KERNEL_VMA)
    {
        *(.data *.data.*)
        . = ALIGN(4K);
    }

    .bss : {
        *(EXCLUDE_FILE(*src/arch/boot.o) .bss .bss.*)
        . = ALIGN(4K);
    }

    .got :
    {
        *(.got)
        . = ALIGN(4K);
    }

    .got.plt :
    {
        *(.got.plt)
        . = ALIGN(4K);
    }

    .data.rel.ro : ALIGN(4K) {
        *(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
        . = ALIGN(4K);
    }

    .gcc_except_table : ALIGN(4K) {
        *(.gcc_except_table)
        . = ALIGN(4K);
    }
}