/*  Architecture Dependent Memory Module
 *  Author: Jianzhong Liu
 *  All rights reserved
 */

pub mod frame;
pub mod page;

use multiboot2;

pub fn init_mem(boot_info: &multiboot2::BootInformation){
    #[cfg(debug_assertions)]
    {
        println!("Initializing memory systems...");
    }
    init_frame_allocator();
    super::log("Memory Frame Allocator initialized.");
    
    remap_kernel();
    super::log("Kernel remapped.");
}

fn init_frame_allocator() {

}

fn remap_kernel(){

}