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

pub fn translate(virtual_address: VirtualAddress) -> Option<PhysicalAddress> {
    let offset = virtual_address % PAGE_SIZE;
    translate_page(Page::containing_address(virtual_address))
        .map(|frame| { frame.start_address() + offset})
}

use mem::frame::Frame;
fn translate_page( page: Page ) -> Option<Frame> {
    use self::entry::HUGE_PAGE;

    let p3 = unsafe{&*self::table::P4}.next_table(page.p4_index());

    unimplemented!()
}

