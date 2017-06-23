; Boot code for TiOS
; x86_64 architecture
; Written by Andrew Jianzhong Liu


    section .text
    bits 32
    
    global start
start:
    mov dword [0xb8000], 0x2f4b2f4f
    jmp $