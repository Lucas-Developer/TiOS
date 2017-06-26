/*  IDT module
 *  Author: Jianzhong Liu
 *  All rights reserved
 */

extern "C" {
    fn set_idt();
}

pub fn load_idt(){
    unsafe{set_idt()};
}