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
use self::frame::*;
use self::page::*;

// Static values

static MEM_INITED : Mutex<bool> = Mutex::new(false);


// Constant values

pub const PAGE_SIZE: usize = 4096; // 4k pages
pub const ENTRY_COUNT: usize = 512; // 512 entries / page table
pub const KERNEL_VMA: usize = 0xffff8000_00000000; // Canonical higher half of kernel

// Type definitions

/// Type for physical addresses.
pub type PhysicalAddress = usize;

/// Type for virtual addresses. 
pub type VirtualAddress = usize; 



// Public structs and interfaces

/// Abstract struct for memory management
pub struct MemoryManager {
    /* Frame allocator, page tables, gdt and others */
    active_table: ActivePageTable,

}

impl MemoryManager {
    // TODO

    pub fn create_new_table() -> InactivePageTable {
        unimplemented!()
    }
}

/// Function to initialize a memory manager
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

    
    let elf_sections_tag = boot_info
                    .elf_sections_tag()
                    .expect("Kernel ELF sections required.");
    let kernel_start = elf_sections_tag
                    .sections()
                    .filter(|s| s.is_allocated())
                    .map(|s| s.addr)
                    .min()
                    .unwrap();
    let kernel_end = elf_sections_tag
                    .sections()
                    .filter(|s| s.is_allocated())
                    .map(|s| s.addr + s.size)
                    .max()
                    .unwrap();
    let multiboot_start = boot_info.start_address();
    let multiboot_end = boot_info.end_address();

    let mut temp_frame_alloc = frame::InitialFrameAllocator::new(
        kernel_start as usize,
        kernel_end as usize,
        multiboot_start as usize,
        multiboot_end as usize,
        boot_info.memory_map_tag().unwrap().memory_areas(),
    );

    let mut active_table = self::page::remap_kernel(boot_info, &mut temp_frame_alloc);
    
    super::log_status("Kernel remapping to higher half", Ok(()));
    
    let frame_alloc = BuddyAllocator::new(temp_frame_alloc, &mut active_table, boot_info);

    super::log_status("Buddy frame allocator initialization", Ok(()));





    
    // TODO: generate a memory manager and return it
    

    unimplemented!()
}