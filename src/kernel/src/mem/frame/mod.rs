/*  Frame module
 *  Author: Jianzhong Liu
 *  All rights reserved
 */

pub mod buddy;
pub mod bitmap;
pub mod temp;

#[derive(Debug, Clone, Copy)]
pub struct Frame {
    num: usize,
    ref_count: usize,
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn allocate_frames(&mut self, num:usize) -> Option<&[Frame]>;
    fn deallocate_frame(&mut self, f: Frame);
}

pub enum FA{
    Area(temp::AreaFrameAllocator),
    Bitmap(bitmap::BitmapAllocator),
    Buddy(buddy::BuddyFrameAllocator),
    Uninitialized,
}



use spin::Mutex;
pub static FRAME_ALLOCATOR : Mutex<FA> = Mutex::new(FA::Uninitialized);