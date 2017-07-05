/*  Paging module 
 *  Author: Andrew Jianzhong Liu
 *  All rights reserved
 */


pub mod table;

use multiboot2;
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Page {
    number: usize,
}

#[derive(Debug, Clone)]
pub struct PageIter{
    start: Page,
    end: Page,
}

use core::ops::Add;

impl Add<usize> for Page {
    type Output = Page;

    fn add(self, rhs: usize) -> Page {
        Page {number : self.number + rhs}
    }
}

impl From<VirtualAddress> for Page {
    fn from(virtual_address: VirtualAddress) -> Page {
        assert!(virtual_address <= 0x0000_7fff_ffff_ffff || virtual_address >= 0xffff_8000_0000_0000,
        "Non-canonical virtual address: {:x}", virtual_address);
        Page{ number: virtual_address / PAGE_SIZE}
    }
}

impl Page {
    pub fn start_address(&self) -> VirtualAddress {
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

pub fn range_inclusive(start: Page, end: Page) -> PageIter {
    assert!(start <= end, "Starting page ({:?}) must not be greater than ending page ({:?})!", start, end);
    PageIter {
        start: start,
        end: end,
    }
}

impl Iterator for PageIter {
    type Item = Page;
    fn next(&mut self) -> Option<Page> {
        if self.start <= self.end {
            let page = self.start;
            self.start.number += 1;
            Some(page)
        }
        else{
            None
        }
    }
}

use super::frame::*;
use self::table::*;
use self::table::entries::*;
use core::ops::{Deref, DerefMut};
use core::ptr::Unique;

pub struct ActivePageTable {
    inner: InnerPageTable,
}

struct InnerPageTable {
    p4: Unique<PageTable<Level4>>, 
}

impl InnerPageTable{
    pub unsafe fn new() -> InnerPageTable {
        InnerPageTable{
            p4: Unique::new(&mut *(self::table::P4_ADDR as *mut _))
        }
    }

    fn p4(&self) -> &PageTable<Level4> {
        unsafe{self.p4.as_ref()}
    }

    fn p4_mut(&mut self) -> &mut PageTable<Level4> {
        unsafe{self.p4.as_mut()}
    }

    pub fn translate(&self, virtual_address: VirtualAddress) -> Option<PhysicalAddress> {
        self.translate_page(Page::from(virtual_address))
                .map(|frame| {frame.start_address() + virtual_address % PAGE_SIZE })
    }

    fn translate_page(&self, page: Page) -> Option<Frame> {
        // If exists in p4
        if let Some(p3_table) = self.p4().next_table(page.p4_index()){
            // If exists in p3
            if let Some(p2_table) = p3_table.next_table(page.p3_index()){
                // If exists in p2
                if let Some(p1_table) = p2_table.next_table(page.p2_index()){
                    // Get the frame referred to by the p1 table
                    return p1_table[page.p1_index()].pointed_frame();
                }
                // 2MiB Huge Page
                else{
                    // Retrieve the p2 table entry
                    let p2_entry = &p2_table[page.p2_index()];
                    // If it points to some frame
                    if let Some(huge_frame) = p2_entry.pointed_frame() {
                        // and contains the huge page flag
                        if p2_entry.flags().contains(HUGE_PAGE){
                            // 2MiB Huge pages must be 2MiB aligned
                            assert!(huge_frame.number % ENTRY_COUNT == 0, "2MiB pages must be 2MiB aligned!");
                            return Some(Frame{
                                number: huge_frame.number + page.p1_index(),
                            });
                        }
                    }
                }
            }
            // 1GiB Huge page
            else{
                let p3_entry = &p3_table[page.p3_index()];
                if let Some(huge_frame) = p3_entry.pointed_frame() {
                    if p3_entry.flags().contains(HUGE_PAGE) {
                        assert!(huge_frame.number %(ENTRY_COUNT * ENTRY_COUNT) == 0, "1GiB pages must be 1GiB aligned!");
                        return Some(Frame{
                            number: huge_frame.number + page.p2_index() *  ENTRY_COUNT + page.p1_index(),
                        });
                    }
                }
            }
        }
        None
    }
}

pub struct InactivePageTable {

}

pub fn remap_kernel<FA>(boot_info: &multiboot2::BootInformation, 
                    allocator: &mut FA) -> ActivePageTable
                    where FA: FrameAllocator { // TODO: return active table
    unimplemented!()   
}