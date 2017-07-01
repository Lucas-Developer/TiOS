
    extern idt64
    extern idt64.pointer
    extern gdt64.code

    global set_isr_gate
    global set_idt
    global set_tss_offset

; Function to set one isr gate for one idt entry
; fn set_isr_gate(num : usize, addr: usize) , registers rdi and rsi
set_isr_gate:
    push rbx
    mov rbx, rdi
    shl rbx, 4 ; Get the byte offset to the entry
    mov rax, idt64
    add rax, rbx ; Get the absolute offset
    mov rbx, rsi ; Move the address of the isr to rbx
    mov word [rax], bx ; First part of entry, offset [0:15]
    add rax, 2

    mov rcx, gdt64.code
    mov word [rax], cx ; Segment selector
    add rax, 2

    mov byte [rax], 0  ; IST
    inc rax

    mov byte [rax], (1 << 7) | (0 << 5) | 0xe
    inc rax

    shr rbx, 16
    mov word [rax], bx ; Second part of offset
    add rax, 2

    shr rbx, 16
    mov dword [rax], ebx; Last part
    add rax, 4

    mov dword [eax], 0

    pop rbx
    ret


; fn set_segment(num : usize, tss_offset: usize ) registers rdi and rsi
set_tss_offset:
    push rbx
    mov rbx, rdi
    shl rbx, 4 ; Get the byte offset to the entry
    mov rax, idt64
    add rax, rbx ; Get the absolute offset
    add rax, 4   ; Get the offset of the tss offset
    mov rcx, rsi
    mov [rax], cx
    pop rbx
    ret

set_idt:
    lidt [idt64.pointer]