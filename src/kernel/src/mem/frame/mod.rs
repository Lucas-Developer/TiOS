/*  Frame module
 *  Author: Jianzhong Liu
 *  All rights reserved
 */

pub mod buddy;

pub struct Frame {
    num: usize,
    ref_count: usize,
}

pub trait FrameAllocator {
    fn allocate_frame() -> Frame;
    fn deallocate_frame(f: Frame);
}

