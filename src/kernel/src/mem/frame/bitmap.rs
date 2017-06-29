/*  Bitmap Frame allocator for TiOS
 *  Author: Jianzhong Liu
 *  All rights reserved
 */

use super::*;
use multiboot2::{MemoryAreaIter, MemoryArea};
use core::convert::{From, Into};

#[repr(C)]
pub struct BitmapAllocator {
    bitmap_addr : u64,
    bitmap_length : u64,
    next_free_frame: u64,
}

#[allow(dead_code, unused_variables)]
impl BitmapAllocator {
    fn new() -> BitmapAllocator {
        
        unimplemented!()
    }
}

#[allow(dead_code, unused_variables)]
impl FrameAllocator for BitmapAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>{
        unimplemented!()
    }

    fn allocate_frames(&mut self, num:usize) -> Option<&[Frame]> {
        unimplemented!()
    }

    fn deallocate_frame(&mut self,f : Frame){
        unimplemented!()
    }
}

impl From<super::temp::AreaFrameAllocator> for BitmapAllocator {
    fn from(tmpalloc: super::temp::AreaFrameAllocator) -> BitmapAllocator {
        unimplemented!()
    }
}