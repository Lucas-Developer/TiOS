/*  Core map module 
 *  Author: Andrew Jianzhong Liu
 *  All rights reserved
 */

use super::super::*;

use multiboot2;

bitflags! {
    flags CoreMapFlags: u8 {
        const IS_ALLOCATED = 1 << 0,
        const IS_KERNEL_FRAME = 1 << 1,

    }
}

#[repr(C,packed)]
#[derive(Debug, Copy, Clone)]
pub struct CoreMapEntry(u64);

// Bits:
// 16 - 63: vma w/o upper 16 bits
// 0: is allocated
// 1: is kernel frame
// 2 - 7 : reserved
// 8 - 15: pid 

impl CoreMapEntry {
    pub fn new(vma: VirtualAddress, flags: CoreMapFlags, pid: u8) -> CoreMapEntry{
        let entry = ((vma as u64) << 16) | (flags.bits() as u64) | ((pid as u64) << 8);
        CoreMapEntry(entry)
    }

    pub fn new_zero() -> CoreMapEntry {
        CoreMapEntry(0x0)
    }
}

pub struct CoreMap {
    entries: &'static mut [CoreMapEntry], // Entries
    size: usize,
}

use core::ops::{Index, IndexMut};

impl Index<usize> for CoreMap {
    type Output = CoreMapEntry;
    
    fn index(&self, index:usize) -> &CoreMapEntry {
        assert!(index < self.size, "Core map indexing out of bounds with index {} in size {}.", index, self.size);
        &self.entries[index]
    }

}

impl IndexMut<usize> for CoreMap {
    fn index_mut(&mut self, index:usize) -> &mut CoreMapEntry {
        assert!(index < self.size, "Core map indexing out of bounds with index {} in size {}.", index, self.size);
        &mut self.entries[index]
    }
}

use super::super::page::table::entries::WRITABLE;

impl CoreMap {
    pub unsafe fn new<A>(mem_areas: multiboot2::MemoryAreaIter,
                         active_table: &mut ActivePageTable,
                         allocator: &mut A)
                         -> CoreMap 
                         where A: FrameAllocator {

        for area in mem_areas.clone(){
            println!("{:?}",area);
        }

        let frame_num = mem_areas.map(|area| area.base_addr + area.length - 1)
            .max().unwrap() as usize / PAGE_SIZE;
        let entry_bytes = frame_num * 8;
        let entry_frame_num = {
            let mut base = entry_bytes / PAGE_SIZE;
            if entry_bytes % PAGE_SIZE != 0{
                base += 1;
            }
            base
        };

        println!("Top frame number = {}, size in bytes = {}, size in frames: {}", frame_num, entry_bytes, entry_frame_num);

        let first_frame = allocator.allocate_frame().expect("No remaining frames");
        let map_addr = first_frame.start_address();
        println!("Initial Frame: {:?}", first_frame);
        
        let mut new_frame = Frame::from(0);
        for i in 1..entry_frame_num {
            new_frame = allocator.allocate_frame().expect("No remaining frames");
        }

        let mut frame_range = super::FrameIter::new(first_frame, new_frame);
        assert!(frame_range.range() == entry_frame_num);

        let flags = WRITABLE;
        for frame in frame_range.clone() {
            active_table.higher_kernel_map(frame, flags, allocator);
        }

        
        for i in 0..frame_num {
            let entry_addr = map_addr + KERNEL_VMA + i * 8;
            println!("{:#x}", unsafe{*(entry_addr as *mut u64)});
        }
        

        println!("Frame range: {:?}", frame_range);

        unimplemented!()
    }

    // Set entry in core map to be unused
    pub fn set_unused(&mut self, frame: Frame){
        unimplemented!()
    }

    pub fn set(&mut self, frame: Frame, flags: CoreMapFlags, pid: u32) {
        unimplemented!()
    }
}