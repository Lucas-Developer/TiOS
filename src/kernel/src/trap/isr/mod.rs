/*  ISR module
 *  Author: Jianzhong Liu
 *  All rights reserved
 */

extern "C" {
    fn set_isr_gate(num : usize, addr: usize);
}

macro_rules! save_scratch_registers {
    () => {
        asm!("push rax
              push rcx
              push rdx
              push rsi
              push rdi
              push r8
              push r9
              push r10
              push r11"
        :::: "intel", "volatile");
    }
}

macro_rules! restore_scratch_registers {
    () => {
        asm!(
        "pop r11
         pop r10
         pop r9
         pop r8
         pop rdi
         pop rsi
         pop rdx
         pop rcx
         pop rax"
         :::: "intel", "volatile"
        );
    }
}

macro_rules! handler {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe{
                save_scratch_registers!();
                asm!("mov rdi, rsp
                      add rdi, 9*8
                      call $0"
                      :: "i" ($name as extern "C" fn (&ExceptionStackFrame)) : "rdi" : "intel", "volatile");
                restore_scratch_registers!();
                asm!("iretq"
                      ::::"intel","volatile");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper as usize
    }}
}

macro_rules! handler_with_error_code {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe{
                save_scratch_registers!();
                asm!("mov rsi, [rsp + 8 * 9] // Get error code
                      mov rdi, rsp 
                      add rdi, 10 * 8 // error frame
                      sub rsp, 8 // Stack alignment
                      call $0
                      add rsp, 8 // Undo stack alignment"
                      :: "i" ($name as extern "C" fn (&ExceptionStackFrame, u64)) 
                      : "rdi", "rsi" : "intel");
                restore_scratch_registers!();
                asm!("add rsp, 8 // Pop error code
                      iretq"
                      ::::"intel","volatile");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper as usize
    }}
}

#[derive(Debug)]
#[repr(C)]
pub struct ExceptionStackFrame {
    instruction_pointer : u64,
    code_segment : u64,
    cpu_flags : u64,
    stack_pointer : u64,
    stack_segment : u64,
}

extern "C" fn breakpoint_handler(stack_frame : &ExceptionStackFrame) {
    let stack_frame = &*stack_frame;
    println!("\nPROCESSOR EXCEPTION: BREAKPOINT at {:#x}\n{:#?}",
        stack_frame.instruction_pointer, stack_frame
    );
    
}

pub fn init_isr(){
    unsafe{set_isr_gate(3,handler!(breakpoint_handler))};
}