; Boot code for TiOS
; x86_64 architecture
; Written by Andrew Jianzhong Liu


    section .text
    bits 32
    
    global start
    extern long_start
start:
    mov esp, stack_top
    mov edi, ebx
    call check_multiboot
    call check_cpuid
    call check_long_mode

    call set_page_tables
    call enable_paging
    lgdt [gdt64.pointer]
    mov dword [0xb8000], 0x2f4b2f4f


    jmp gdt64.code:long_start

; Utility functions for activating provisional paging

enable_paging:
    ; Set p4 table address in cr3
    mov eax, p4_table
    mov cr3, eax

    ; Set PAE
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax

    ; Set long mode
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr

    ; Set paging on
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    ret


set_page_tables:
    mov eax, p3_table
    or eax, 0b11 ; present + writable
    mov [p4_table], eax

    mov eax, p2_table
    or eax, 0b11 ; present + writable
    mov [p3_table], eax

    mov ecx, 0 ; Counter variable
.map_p2_table:
    mov eax, 0x200000 ; 2MiB Page
    mul ecx
    or eax, 0b10000011 ; huge + writable + present
    mov [p2_table + ecx * 8], eax
    inc ecx
    cmp ecx, 512
    jne .map_p2_table
    ret




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
    resb 4096*4
stack_top:

    section .rodata
gdt64:
    dq 0 ; zero entry
.code: equ $ - gdt64 ; new
    dq (1<<43) | (1<<44) | (1<<47) | (1<<53) ; code segment
.pointer:
    dw $ - gdt64 - 1
    dq gdt64