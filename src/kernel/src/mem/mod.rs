/*  Architecture Dependent Memory Module
 *  Author: Jianzhong Liu
 *  All rights reserved
 */

pub mod frame;
pub mod page;
pub use self::page::test_paging;
use multiboot2;

pub fn init_mem(boot_info: &multiboot2::BootInformation){
    #[cfg(debug_assertions)]
    {
        println!("Initializing memory systems...");
    }


    use self::frame::temp::*;
    let elf_sections_tag = boot_info.elf_sections_tag().expect("Kernel ELF sections required.");
    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
        .min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
        .max().unwrap();
    let multiboot_start = boot_info.start_address();
    let multiboot_end = boot_info.end_address();
    let mut frame_allocator : AreaFrameAllocator = AreaFrameAllocator::new(kernel_start as usize,
        kernel_end as usize, multiboot_start as usize, multiboot_end as usize, 
        boot_info.memory_map_tag().unwrap().memory_areas());

    super::log("Memory Frame Allocator initialized.");

    self::page::remap_kernel(&mut frame_allocator, boot_info);
    super::log("Kernel remapped.");
}

