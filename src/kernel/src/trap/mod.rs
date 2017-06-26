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
    isr::init_isr();
    x86_64::instructions::interrupts::int3();
}