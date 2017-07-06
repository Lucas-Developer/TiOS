/*  Frame management module
 *  Author: Andrew Jianzhong Liu
 *  All rights reserved
 */

// External Imports
use multiboot2::{MemoryAreaIter, MemoryArea};
use super::*;

mod coremap;

// Public struct for Frames
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Frame {
    pub number: usize,
}

// Iterators for frames
pub struct FrameIter {
    start: Frame,
    end: Frame,
}

impl Iterator for FrameIter {
    type Item = Frame;
    fn next(&mut self) -> Option<Frame> {
        if self.start <= self.end {
            let frame = self.start.clone();
            self.start.number += 1;
            Some(frame)
        }
        else{
            None
        }
    }
}

// Frame related functions
impl Frame {

    // Get the start address of a frame
    pub fn start_address(&self) -> PhysicalAddress {
        self.number * PAGE_SIZE
    }

    // Get a clone of the current frame
    pub fn clone(&self) -> Frame {
        Frame {
            number: self.number,
        }
    }

    pub fn range_inclusive(start: Frame, end:Frame) -> FrameIter {
        FrameIter::new(start,end)
    }
}

// Get a Frame for a Physical Address
impl From<PhysicalAddress> for Frame {
    fn from(physical_address: PhysicalAddress) -> Self {
        Frame {
            number: physical_address / PAGE_SIZE,
        }
    }
}

impl FrameIter {
    // Get a new FrameIter
    pub fn new(start: Frame, end:Frame) -> FrameIter {
        FrameIter {
            start: start,
            end: end,
        }
    }
}

/// Trait defining interface for different frame allocators
pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn allocate_contiguous_frames(&mut self, num: usize) -> Option<FrameIter>;
    fn deallocate_frame(&mut self, frame: Frame);
}

#[derive(Debug)]
pub struct InitialFrameAllocator {
    areas: MemoryAreaIter,
    current_area: Option<&'static MemoryArea>,
    next_frame: Frame,
    kernel_start: Frame,
    kernel_end: Frame,
    multiboot_start: Frame,
    multiboot_end: Frame,
    freed_frames: [Option<usize>; 8],
}

impl InitialFrameAllocator {
    pub fn new(kernel_start:usize, kernel_end: usize,
                multiboot_start: usize, multiboot_end: usize,
                memory_areas: MemoryAreaIter) -> InitialFrameAllocator {

        let mut allocator = InitialFrameAllocator {
            areas: memory_areas,
            current_area: None,
            next_frame: Frame::from(0 as PhysicalAddress),
            kernel_start: Frame::from(kernel_start as PhysicalAddress),
            kernel_end: Frame::from(kernel_end as PhysicalAddress),
            multiboot_start: Frame::from(multiboot_start as PhysicalAddress),
            multiboot_end: Frame::from(multiboot_end as PhysicalAddress),
            freed_frames: [None;8],
        };
        allocator.select_next_area();
        allocator
    }

    fn select_next_area(&mut self){
        self.current_area = self.areas.clone().filter(|area| {
            Frame::from((area.base_addr + area.length - 1) as PhysicalAddress) >= self.next_frame
        }).min_by_key(|area| area.base_addr);

        if let Some(area) = self.current_area {
            let start_frame = Frame::from(area.base_addr as PhysicalAddress);
            if self.next_frame < start_frame {
                self.next_frame = start_frame;
            }
        }
    }
}

impl FrameAllocator for InitialFrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
        if let Some(area) = self.current_area {
            let frame = self.next_frame.clone();
            let area_last_frame = 
                Frame::from((area.base_addr + area.length - 1) as PhysicalAddress);
            if frame > area_last_frame {
                self.select_next_area();
            }
            else if frame >= self.kernel_start && frame <= self.kernel_end {
                self.next_frame = Frame{ number: self.kernel_end.number + 1 };
            }
            else if frame >= self.multiboot_start && frame <= self.multiboot_end {
                self.next_frame = Frame{ number: self.multiboot_end.number + 1 };
            }
            else {
                self.next_frame.number += 1;
                return Some(frame);
            }
            self.allocate_frame()
        }
        else{
            None
        }
    }

    fn allocate_contiguous_frames(&mut self, num:usize) -> Option<FrameIter> {
        unimplemented!() // DO NOT USE DURING INITIALIZATION
    }

    fn deallocate_frame(&mut self, frame: Frame){
        for pos in self.freed_frames.iter_mut(){
            if pos.is_none() {
                *pos = Some(frame.number);
                return;
            }
        }
        panic!("Too many frames to free during initialization!");
    }
}

pub struct BuddyAllocator {
    coremap: coremap::CoreMap,
    // frame level
    // 2x frame level
    // 4x frame level
    // 8x frame level
}

// Convert Initial Allocator to Core map allocator
impl BuddyAllocator {
    fn from(init_frame_alloc: InitialFrameAllocator,
            active_table: &mut ActivePageTable,
            ) -> BuddyAllocator {
        //let coremap = unsafe{coremap::CoreMap::new()};

        println!("Initial frame allocator: {:?}",init_frame_alloc);

        unimplemented!()
    }
}

impl FrameAllocator for BuddyAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
        unimplemented!()
    }

    fn allocate_contiguous_frames(&mut self, num:usize) -> Option<FrameIter> {
        unimplemented!()
    }

    fn deallocate_frame(&mut self, frame: Frame){
        unimplemented!()
    }
}

