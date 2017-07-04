/*  Frame management module
 *  Author: Andrew Jianzhong Liu
 *  All rights reserved
 */

pub struct Frame {
    number: usize,
}

pub trait FrameAllocator {
    
    kernel_start: Frame,
    kernel_end: Frame,
    multiboot_start: Frame,
    multiboot_end: Frame,
}