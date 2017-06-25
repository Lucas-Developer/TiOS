; Long mode (64-bit mode) Boot code for TiOS
; x86_64 architecture
; Written by Andrew Jianzhong Liu

    section .text
    bits 64

    global long_start

    ; From util/console.asm
    extern clear_console
    extern print_char

long_start:
    call clear_console
    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax
    jmp $