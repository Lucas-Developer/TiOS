; Long mode (64-bit mode) Boot code for TiOS
; x86_64 architecture
; Written by Andrew Jianzhong Liu

    section .text
    bits 64

    global bootstrap_long

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

    mov rax, rust_start
    jmp [rax]
    cli
    hlt

    section .bootstrap64
    bits 64
bootstrap_long:
    mov rax, long_start
    jmp [rax]