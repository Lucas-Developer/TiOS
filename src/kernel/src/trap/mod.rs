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
    super::log("Interrupt Descriptor Table loaded.");
    isr::init_isr();
    super::log("Initial interrupt service routines loaded.");
    //x86_64::instructions::interrupts::int3();
}