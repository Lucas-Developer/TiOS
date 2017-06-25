; Long mode (64-bit mode) Boot code for TiOS
; x86_64 architecture
; Written by Andrew Jianzhong Liu

    section .text
    bits 64

    global long_start

    ; From util/console.asm
    extern clear_console
    extern print_char

    ; From Rust code
    extern rust_start

long_start:
    cli
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    call clear_console
    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax
    jmp rust_start