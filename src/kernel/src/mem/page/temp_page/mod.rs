/*  Temporary Page Module
 *  Author: Jianzhong Liu
 */

use super::Page;

#[derive(Debug)]
pub struct TemporaryPage {
    page: Page,
    allocator: TinyAllocator,
}

use super::{ActivePageTable, VirtualAddress};
use super::super::frame::Frame;
use super::table::{Table, Level1};
use mem::frame::FrameAllocator;

impl TemporaryPage {

    pub fn new<A>(page: Page, allocator: &mut A) -> TemporaryPage
        where A: FrameAllocator {
            println!("{:?}",page);
        TemporaryPage {
            page: page,
            allocator: TinyAllocator::new(allocator),
        }
    }

    pub fn map(&mut self, frame: Frame, active_table: &mut ActivePageTable) -> VirtualAddress {
        use super::entry::WRITABLE;


        assert!(active_table.translate_page(self.page).is_none(),
                "temporary page is already mapped");


        active_table.map_to(self.page, frame, WRITABLE, &mut self.allocator);

        self.page.start_address()
    }

    pub fn unmap(&mut self, active_table: &mut ActivePageTable) {
        active_table.unmap(self.page, &mut self.allocator)
    }

    pub fn map_table_frame(&mut self,
                       frame: Frame,
                       active_table: &mut ActivePageTable)
                       -> &mut Table<Level1> {
        unsafe { &mut *(self.map(frame, active_table) as *mut Table<Level1>) }
    }
}

#[derive(Debug)]
struct TinyAllocator([Option<Frame>;3]);



impl FrameAllocator for TinyAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
        for frame_option in &mut self.0 {
            if frame_option.is_some() {
                return frame_option.take();
            }
        }
        None
    }

    fn allocate_frames(&mut self, num:usize) -> Option<&[Frame]> {
        unimplemented!()
    }

    fn deallocate_frame(&mut self, frame: Frame) {
        for frame_option in &mut self.0 {
            if frame_option.is_none() {
                *frame_option = Some(frame);
                return;
            }
        }
        panic!("Tiny allocator can hold only 3 frames.");
    }
    
}

impl TinyAllocator {
    fn new<A>(allocator: &mut A) -> TinyAllocator
        where A: FrameAllocator
    {
        let mut f = || allocator.allocate_frame();
        let frames = [f(), f(), f()];
        TinyAllocator(frames)
    }


}