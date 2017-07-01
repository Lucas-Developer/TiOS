/*  Frame module
 *  Author: Jianzhong Liu
 *  All rights reserved
 */

pub mod buddy;
pub mod bitmap;
pub mod temp;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Frame {
    pub num: usize,
}

pub struct FrameIter {
    start: Frame,
    end: Frame,
}

impl Iterator for FrameIter {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        if self.start <= self.end {
            let frame = self.start.clone();
            self.start.num += 1;
            Some(frame)
        } else {
            None
        }
    }
 }

use super::page::PhysicalAddress;
impl Frame{
    pub fn containing_address(address: usize) -> Frame{
        Frame{ num: address/ super::page::PAGE_SIZE }
    }

    pub fn range_inclusive(start: Frame, end: Frame) -> FrameIter {
        FrameIter {
            start: start,
            end: end,
        }
    }

    pub fn start_address(&self) -> PhysicalAddress {
        self.num * super::page::PAGE_SIZE
    }

    pub fn clone(&self) -> Frame{
        Frame{
            num: self.num,
        }
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn allocate_frames(&mut self, num:usize) -> Option<&[Frame]>;
    fn deallocate_frame(&mut self, f: Frame);
}
