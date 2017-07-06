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
pub struct CoreMapEntry{
    page: Page,
    pid: u8,
    flag: u8,
}

impl CoreMapEntry {

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

impl CoreMap {
    pub unsafe fn new<A>(mem_areas: multiboot2::MemoryAreaIter,
                         active_table: &mut ActivePageTable,
                         allocator: &mut A)
                         -> CoreMap 
                         where A: FrameAllocator {

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