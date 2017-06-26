/*  Bitmap Frame allocator for TiOS
 *  Author: Jianzhong Liu
 *  All rights reserved
 */

use super::*;
use multiboot2::{MemoryAreaIter, MemoryArea};

pub struct BitmapAllocator {
    
}

#[allow(dead_code, unused_variables)]
impl BitmapAllocator {
    fn new() -> BitmapAllocator {
        
        unimplemented!()
    }

    fn get_next_frame(&mut self) -> Option<Frame> {
        unimplemented!()
    }
}

#[allow(dead_code, unused_variables)]
impl FrameAllocator for BitmapAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>{
        self.get_next_frame()
    }

    fn allocate_frames(&mut self, num:usize) -> Option<&[Frame]> {
        unimplemented!()
    }

    fn deallocate_frame(&mut self,f : Frame){
        unimplemented!()
    }
}