KERNEL_VMA = 0xffff800000000000;
KERNEL_LMA = 1M;

ENTRY(start)
OUTPUT_FORMAT(elf64-x86-64)

SECTIONS {
    . = KERNEL_LMA;

    .boot : 
    {
        KEEP(*(.multiboot_header))
    }

    .bootstrap :
    {
        *(.text)
    }

    .bss.bootstrap : 
    {
        boot/boot.o (.bss)
    }

    .rodata.bootstrap :
    {
        boot/boot.o (.rodata)
    }

    . += KERNEL_VMA;

    .rodata : AT(ADDR(.rodata) - KERNEL_VMA)
    {
        *(EXCLUDE_FILE(*boot/boot.o) .rodata .rodata.*)
    }

    .data : AT(ADDR(.data) - KERNEL_VMA)
    {
        *(.data *.data.*)
    }

    .bss : AT(ADDR(.bss) - KERNEL_VMA)
    {
        *(EXCLUDE_FILE(*boot/boot.o) .bss .bss.*)
    }
}