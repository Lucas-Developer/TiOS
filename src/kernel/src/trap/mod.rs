/*  Trap handler module
 *  Author: Jianzhong Liu
 *  All rights reserved
 */


#[macro_use]
pub mod isr;

pub mod idt;

use x86_64;

use self::idt::IdtFrontEnd;

lazy_static!{
    static ref Idt : IdtFrontEnd = {
        let idt = IdtFrontEnd{};
        idt
    };
}

pub fn init_trap() {
    Idt.load_idt();
    super::log_status("Interrupt Descriptor Table Initialization ...........  ", Ok(()));

    //super::log_status("Initial interrupt service routines load .............  ", Ok(()));
    #[cfg(debug_assertions)]
    {
        x86_64::instructions::interrupts::int3();
        //unsafe {
        //    asm!("mov dx, 0; div dx" ::: "ax", "dx" : "volatile", "intel")
        //}
    }
}