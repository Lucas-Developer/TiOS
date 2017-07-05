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

pub struct ActivePageTable {


}

pub struct InactivePageTable {

}

pub fn remap_kernel<FA>(boot_info: &multiboot2::BootInformation, 
                    allocator: &mut FA) -> ActivePageTable
                    where FA: FrameAllocator { // TODO: return active table
    unimplemented!()   
}