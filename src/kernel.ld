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
        . = ALIGN(4K);
    }

    .bss.bootstrap : 
    {
        *(.bss.bootstrap)
        . = ALIGN(4K);
    }

    .rodata.bootstrap :
    {
        *(.rodata.bootstrap)
        . = ALIGN(4K);
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

    .bss : AT(ADDR(.bss) - KERNEL_VMA)
    {
        *(EXCLUDE_FILE(*src/arch/boot.o) .bss .bss.*)
        . = ALIGN(4K);
    }

    /DISCARD/ :
    {
        *(.comment)
    }
}