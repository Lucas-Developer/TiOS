/*  Frame module
 *  Author: Jianzhong Liu
 *  All rights reserved
 */

pub mod buddy;
pub mod bitmap;
pub mod temp;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Frame {
    num: usize,
}

impl Frame{
    fn containing_address(address: usize) -> Frame{
        Frame{ num: address/ super::page::PAGE_SIZE }
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn allocate_frames(&mut self, num:usize) -> Option<&[Frame]>;
    fn deallocate_frame(&mut self, f: Frame);
}

pub enum FA{
    Bitmap(bitmap::BitmapAllocator),
    Buddy(buddy::BuddyFrameAllocator),
    Uninitialized,
}



use spin::Mutex;
pub static FRAME_ALLOCATOR : Mutex<FA> = Mutex::new(FA::Uninitialized);