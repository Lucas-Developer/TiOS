/*  IDT module
 *  Author: Jianzhong Liu
 *  All rights reserved
 */

use super::*;
use super::isr::*;

extern "C" {
    fn set_idt();
    fn set_isr_gate(num : usize, addr: usize);
}

pub struct IdtFrontEnd {

}

impl IdtFrontEnd {
    pub fn load_idt(&self) {
        unsafe {set_idt()};
        self.init_isr();
    }

    fn init_isr(&self){
        // TODO: Fill in all default handlers
        self.set_isr(0,handler!(default_handler));
        self.set_isr(3,handler!(breakpoint_handler));
    }
    
    pub fn set_isr(&self, gate: usize, handler_addr:usize){ // TODO: mask gate in an enum and hander_addr in a function pointer
        unsafe{set_isr_gate(gate,handler_addr)};
    }
}
