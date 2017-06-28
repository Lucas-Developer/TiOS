/*  page module
 *  Author: Jianzhong Liu
 *  All rights reserved
 */

pub const PAGE_SIZE: usize = 4096;

const ENTRY_COUNT: usize = 512;

pub mod entry;
pub mod table;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

pub struct Page {
    number : usize,
}

impl Page{
    pub fn containing_address(virtual_address: VirtualAddress) -> Page {
        assert!(virtual_address <0x0000_8000_0000_0000 || 
            virtual_address >= 0xffff_8000_0000_0000, 
            "invalid virtual address: 0x{:x}", virtual_address);
        Page{ number: virtual_address / PAGE_SIZE }
    }

    fn start_address(&self) -> usize {
        self.number * PAGE_SIZE
    }

    fn p4_index(&self) -> usize {
        (self.number >> 27) & 0o777
    }

    fn p3_index(&self) -> usize {
        (self.number >> 18) & 0o777
    }

    fn p2_index(&self) -> usize {
        (self.number >> 9) & 0o777
    }

    fn p1_index(&self) -> usize {
        (self.number >> 0) & 0o777
    }
}



use mem::frame::Frame;


use mem::page::entry::EntryFlags;
use mem::frame::FrameAllocator;


use self::table::{Table, Level4};
use core::ptr::Unique;
use self::entry::HUGE_PAGE;

pub struct ActivePageTable {
    p4: Unique<Table<Level4>>,
}

impl ActivePageTable {
    pub unsafe fn new() -> ActivePageTable {
        ActivePageTable {
            p4: Unique::new(table::P4),
        }
    }

    fn p4(&self) -> &Table<Level4> {
        unsafe { self.p4.as_ref() }
    }

    fn p4_mut(&mut self) -> &mut Table<Level4> {
        unsafe { self.p4.as_mut() }
    }

    pub fn translate(&self, virtual_address: VirtualAddress) -> Option<PhysicalAddress> {
        let offset = virtual_address % PAGE_SIZE;
        self.translate_page(Page::containing_address(virtual_address))
            .map(|frame| { frame.start_address() + offset})
    }

    fn translate_page(&self, page: Page) -> Option<Frame> {
    

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
                    assert!(start_frame.num % ENTRY_COUNT == 0);
                    return Some(Frame{
                        num: start_frame.num + page.p1_index(),
                    });
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
}