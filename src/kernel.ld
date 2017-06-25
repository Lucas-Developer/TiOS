ENTRY(start)

SECTIONS {
    . = 1M;

    .boot : 
    {
        KEEP(*(.boot))
    }

    .text : {
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data *.data.*)
    }

    .bss : {
        *(.bss .bss.*)
    }
}