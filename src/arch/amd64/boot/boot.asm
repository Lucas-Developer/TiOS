; Boot code for TiOS
; x86_64 architecture
; Written by Andrew Jianzhong Liu


    section .text
    bits 32
    
    global start
    extern long_start
start:
    mov esp, stack_top
    call check_multiboot
    call check_cpuid
    call check_long_mode

    mov dword [0xb8000], 0x2f4b2f4f


    jmp $

















; Utility functions to check for corresponding features of the CPU

; Check for multiboot
check_multiboot:
    cmp eax, 0x36d76289
    jne .no_multiboot
    ret
.no_multiboot:
    mov al, "0"
    jmp print_err

; Check cpuid
check_cpuid:
    pushfd
    pop eax
    mov ecx, eax
    xor eax, 1<<21
    push eax
    popfd

    pushfd
    pop eax
    
    push ecx
    popfd

    cmp eax, ecx
    je .no_cpuid
    ret
.no_cpuid:
    mov al, "1"
    jmp print_err

; Check for long mode
check_long_mode:
    mov eax, 0x80000000
    cpuid
    cmp eax, 0x80000001
    jl .no_long_mode

    mov eax, 0x80000001
    cpuid
    test edx, 1 << 29
    jz .no_long_mode
    ret
.no_long_mode:
    mov al, "2"
    jmp print_err

; Internal function to print an error message to the console
print_err:
    mov dword [0xb8000], 0x4f524f45
    mov dword [0xb8004], 0x4f3a4f52
    mov dword [0xb8008], 0x4f204f20
    mov byte  [0xb800a], al
    hlt


    section .bss
    align 4096
p4_table:
    resb 4096
p3_table:
    resb 4096
p2_table:
    resb 4096
stack_bottom:
    resb 64
stack_top: