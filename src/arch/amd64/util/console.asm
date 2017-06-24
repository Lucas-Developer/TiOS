
global print_char
global clear_console

section .text
bits 64

; Internal function to clear screen, completed

clear_console:
    push rcx
    push rax
    
    mov qword rcx, 0x0 ; Init 
    mov qword rax, [console_width] ; Move value of console_width to rax
    mul qword [console_height] ; Multiply it with console_height
    jmp clear_console.compare_counter
.clear_loop:
    mov dword [0xb8000 + 2*rcx], 0x00000000 ; Clear console
    inc rcx
.compare_counter:
    cmp rcx, rax
    jl .clear_loop

    pop rax
    pop rcx
    ret


; Internal function to scroll down one line

scroll_one_line: ; Scroll down one line
    push rcx ; Save registers used within this function 
    push rax
    push rbx

    mov qword rcx, 0x0;
    mov qword rax, [console_height]
    dec rax
    mul qword [console_width]
    jmp scroll_one_line.compare_counter
.scroll_loop:
    mov dword ebx, [0xb80A0 + 2*rcx]
    mov dword [0xb8000 + 2*rcx], ebx
    inc rcx
    ;inc rcx
.compare_counter:
    cmp rcx, rax
    jl .scroll_loop
    mov dword [current_position], 0xb8000 + 24 * 80 * 2

    mov qword rcx, 0x0;
.clear_last_line:
   
    mov word [0xb8f00 + rcx * 2] , 0x00
    inc rcx
    

    cmp rcx, 0xA0
    jl .clear_last_line

    pop rbx ; Restore registers before call
    pop rax
    pop rcx
    ret

; External function to print out a character with a color

print_char: ; void print_char(short word);
    push rax
    push rbx
    push rcx

    mov dword eax, [current_position] ; Get the address to print to
    mov qword rbx, rdi ; Move the first argument
    cmp bl, 0x0a
    je .print_new_line
    cmp bl, 0x09
    je .print_tab
    cmp bl, 0xd
    je .print_cr
    mov word [eax], bx
    add eax, 2 ; Add 2 to the address
    cmp eax, 0xb8000 + 25 * 80 * 2
    je .print_new_line
    mov dword [current_position], eax
.end:

    pop rcx
    pop rbx
    pop rax
    ret
.print_new_line:
    call scroll_one_line
    jmp .end
.print_tab:
    mov qword rcx, 0
.print_tab_loop:
    mov qword rdi, 0x20
    call print_char
    inc rcx
    cmp rcx, 4
    jl .print_tab_loop
    jmp .end
.print_cr:
    mov dword [current_position], 0xb8000 + 24 * 80 * 2
    jmp .end



section .data

console_width:
    dq 80
console_height:
    dq 25
current_position:
    dq 0xb8000 + 24 * 80 * 2
