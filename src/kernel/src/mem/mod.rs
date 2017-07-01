/*  Architecture Dependent Memory Module
 *  Author: Jianzhong Liu
 *  All rights reserved
 */

pub mod frame;
pub mod page;
pub use self::page::test_paging;
pub mod stack_allocator;
use multiboot2;
use spin::Mutex;
use core::ops::DerefMut;

static MEM_INITED : Mutex<bool> = Mutex::new(false);

pub fn init_mem(boot_info: &multiboot2::BootInformation) -> MemoryController{

    // Check if init_mem has been called multiple times
    {
        let mut mem_init_done_stat_guard = MEM_INITED.lock();
        if let true = *mem_init_done_stat_guard {
            panic!("mem::init_mem() can only be called once!");
        }
        else{
            *mem_init_done_stat_guard = true;
        }
    }

    #[cfg(debug_assertions)]
    {
        println!("Initializing memory systems...");
    }


    use self::frame::temp::*;
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
    let mut frame_allocator : AreaFrameAllocator = 
                    AreaFrameAllocator::new
                        (kernel_start as usize,
                         kernel_end as usize, 
                         multiboot_start as usize, 
                         multiboot_end as usize, 
                         boot_info
                            .memory_map_tag()
                            .unwrap()
                            .memory_areas());

    super::log_status("Memory Frame Allocator Initialization ...............  ", Ok(()));

    let mut active_table = self::page::remap_kernel(
                                &mut frame_allocator, boot_info);
    super::log_status("Kernel Remapping ....................................  ", Ok(()));
    
    use self::page::Page;
    use hole_allocator::{HEAP_START, HEAP_SIZE};
    
    let heap_start_page = Page::containing_address(HEAP_START);
    let heap_end_page = Page::containing_address(HEAP_START + HEAP_SIZE-1);

    for page in Page::range_inclusive(heap_start_page, heap_end_page) {
        active_table.map(page, page::WRITABLE, &mut frame_allocator);
    }

    let stack_allocator = {
        let stack_alloc_start = heap_end_page + 1;
        let stack_alloc_end = stack_alloc_start + 100;
        let stack_alloc_range = Page::range_inclusive(stack_alloc_start,
                                                      stack_alloc_end);
        stack_allocator::StackAllocator::new(stack_alloc_range)
    };

    MemoryController {
        active_table: active_table,
        frame_allocator: frame_allocator,
        stack_allocator: stack_allocator,
    }
}

pub use self::stack_allocator::Stack;


pub struct MemoryController {
    active_table: page::ActivePageTable,
    frame_allocator: frame::temp::AreaFrameAllocator,
    stack_allocator: stack_allocator::StackAllocator,
}

pub struct RuntimeMemoryController {
    active_table: page::ActivePageTable,
    frame_allocator: frame::bitmap::BitmapAllocator,
    stack_allocator: stack_allocator::StackAllocator,
}

impl MemoryController {
    pub fn alloc_stack(&mut self, size_in_pages: usize) -> Option<Stack> {
        let &mut MemoryController { ref mut active_table,
                                    ref mut frame_allocator,
                                    ref mut stack_allocator } = self;
        stack_allocator.alloc_stack(active_table, frame_allocator,
                                    size_in_pages)
    }
}

use self::frame::bitmap::BitmapAllocator;

impl From<MemoryController> for RuntimeMemoryController {
    fn from(mem_ctrl: MemoryController) -> Self {
        RuntimeMemoryController{
            active_table : mem_ctrl.active_table,
            frame_allocator: BitmapAllocator::from(mem_ctrl.frame_allocator),
            stack_allocator: mem_ctrl.stack_allocator,
        }
    }
}