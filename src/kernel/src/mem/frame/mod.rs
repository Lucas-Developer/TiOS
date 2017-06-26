/*  Frame module
 *  Author: Jianzhong Liu
 *  All rights reserved
 */

pub mod buddy;
pub mod bitmap;

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

