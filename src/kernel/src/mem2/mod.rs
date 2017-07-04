/*  Memory management module 
 *  Author: Andrew Jianzhong Liu
 *  All rights reserved
 */

// Submodules
mod frame;
mod page;


// External imports
use multiboot2::BootInformation;
use spin::Mutex;


// Static values

static MEM_INITED : Mutex<bool> = Mutex::new(false);


// Public structs and interfaces

pub struct MemoryManager {
    /* Frame allocator, page tables, gdt and others */
}

pub fn init_mem(boot_info : &BootInformation) -> MemoryManager {
    {
        let mut mem_init_stat_guard = MEM_INITED.lock();
        if let true = *mem_init_stat_guard {
            panic!("Memory initialization can only be invoked once!");
        }
        else {
            *mem_init_stat_guard = true;
        }
    }

    
    
    

    unimplemented!()
}