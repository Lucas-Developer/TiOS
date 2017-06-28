

use mem::frame::Frame;
use mem::page::entry::EntryFlags;
use mem::frame::FrameAllocator;
use super::table::{Table, Level4};
use core::ptr::Unique;
use mem::page::entry::{HUGE_PAGE,WRITABLE,PRESENT};
use mem::page::temp_page::TemporaryPage;
use super::VirtualAddress;
use super::PhysicalAddress;
use mem::page::*;

pub struct Mapper {
    p4: Unique<Table<Level4>>,
}

impl Mapper {
    pub unsafe fn new() -> Mapper {
        Mapper {
            p4: Unique::new(super::table::P4),
        }
    }

    pub fn p4(&self) -> &Table<Level4> {
        unsafe { self.p4.as_ref() }
    }

    pub fn p4_mut(&mut self) -> &mut Table<Level4> {
        unsafe { self.p4.as_mut() }
    }

    pub fn translate(&self, virtual_address: VirtualAddress) -> Option<PhysicalAddress> {
        let offset = virtual_address % PAGE_SIZE;
        self.translate_page(Page::containing_address(virtual_address))
            .map(|frame| { frame.start_address() + offset})
    }

    #[no_mangle]
    pub fn translate_page(&self, page: Page) -> Option<Frame> {
        let p3 = self.p4().next_table(page.p4_index());
    let huge_page = || { // 1GiB or 2 MiB Pages ?
        p3.and_then(|p3| {
            let p3_entry = &p3[page.p3_index()];
            if let Some(start_frame) = p3_entry.pointed_frame() {
                if p3_entry.flags().contains(HUGE_PAGE) {
                    assert!(start_frame.num % (ENTRY_COUNT * ENTRY_COUNT) == 0);
                    return Some(Frame{
                        num: start_frame.num + page.p2_index() * ENTRY_COUNT + page.p1_index(),
                    });
                }
            }
            if let Some(p2) = p3.next_table(page.p3_index()) {
                let p2_entry = &p2[page.p2_index()];

                if let Some(start_frame) = p2_entry.pointed_frame(){
                    if p2_entry.flags().contains(HUGE_PAGE) {

                        assert!(start_frame.num % ENTRY_COUNT == 0);
                        return Some(Frame{
                            num: start_frame.num + page.p1_index(),
                        });
                    }
                }
            }
            None
        })
    };

    p3.and_then(|p3| p3.next_table(page.p3_index()))
        .and_then(|p2| p2.next_table(page.p2_index()))
        .and_then(|p1| p1[page.p1_index()].pointed_frame())
        .or_else(huge_page)
    }

    pub fn map_to<A> (&mut self, page: Page, 
                  frame: Frame, 
                  flags: EntryFlags, 
                  allocator : &mut A)
                    where A: FrameAllocator {
        let mut p3 = self.p4_mut().next_table_create(page.p4_index(), allocator);
        let mut p2 = p3.next_table_create(page.p3_index(), allocator);
        let mut p1 = p2.next_table_create(page.p2_index(), allocator);
        assert!(p1[page.p1_index()].is_unused());
        use mem::page::entry::PRESENT;
        p1[page.p1_index()].set(frame, flags | PRESENT);
    }

    pub fn map<A>(&mut self, page: Page, flags: EntryFlags, allocator: &mut A)
        where A: FrameAllocator {
        let frame = allocator.allocate_frame().expect("out of memory");
        self.map_to(page, frame, flags, allocator)
    }

    pub fn identity_map<A>(&mut self,
                            frame: Frame,
                            flags: EntryFlags,
                            allocator: &mut A)
                            where A: FrameAllocator{
        let page = Page::containing_address(frame.start_address());
        self.map_to(page, frame, flags, allocator)
    }

    pub fn unmap<A> (&mut self, page: Page, allocator: &mut A) 
                            where A: FrameAllocator {
        assert!(self.translate(page.start_address()).is_some());

        let p1 = self.p4_mut()
                     .next_table_mut(page.p4_index())
                     .and_then(|p3| p3.next_table_mut(page.p3_index()))
                     .and_then(|p2| p2.next_table_mut(page.p2_index()))
                     .expect("mapping code does not support huge pages");
        let frame = p1[page.p1_index()].pointed_frame().unwrap();
        p1[page.p1_index()].set_unused();

        // TODO free p(1,2,3) table if empty

        use x86_64::instructions::tlb;
        use x86_64::VirtualAddress;
        tlb::flush(VirtualAddress(page.start_address()));
        allocator.deallocate_frame(frame);
    }
}