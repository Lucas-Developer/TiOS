/*  Trap handler module
 *  Author: Jianzhong Liu
 *  All rights reserved
 */

pub mod idt;
#[macro_use]
pub mod isr;
use x86_64;

pub fn init_trap() {
    idt::load_idt();
    super::log_status("Interrupt Descriptor Table Initialization ...........  ", Ok(()));
    isr::init_isr();
    super::log_status("Initial interrupt service routines load .............  ", Ok(()));
    #[cfg(debug_assertions)]
    {
        x86_64::instructions::interrupts::int3();
    }
}