/*  page module
 *  Author: Jianzhong Liu
 *  All rights reserved
 */

pub const PAGE_SIZE: usize = 4096;

const ENTRY_COUNT: usize = 512;

pub mod entry;
pub mod table;
pub mod temp_page;
pub mod mapper;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Page {
    number : usize,
}

use core::ops::Add;

impl Add<usize> for Page {
    type Output = Page;

    fn add(self, rhs: usize) -> Page {
        Page { number: self.number + rhs }
    }
}

impl Page{
    pub fn containing_address(virtual_address: VirtualAddress) -> Page {
        assert!(virtual_address <0x0000_8000_0000_0000 || 
            virtual_address >= 0xffff_8000_0000_0000, 
            "invalid virtual address: 0x{:x}", virtual_address);
        Page{ number: virtual_address / PAGE_SIZE }
    }

    pub fn start_address(&self) -> usize {
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

    pub fn range_inclusive(start: Page, end: Page) -> PageIter {
        PageIter {
            start: start,
            end: end,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PageIter {
    start: Page,
    end: Page,
}

impl Iterator for PageIter {
    type Item = Page;

    fn next(&mut self) -> Option<Page> {
        if self.start <= self.end {
            let page = self.start;
            self.start.number += 1;
            Some(page)
        } else {
            None
        }
    }
}


use mem::frame::Frame;
pub use mem::page::entry::{HUGE_PAGE,WRITABLE,PRESENT};
use mem::frame::FrameAllocator;
use mem::page::entry::EntryFlags;
use mem::page::temp_page::TemporaryPage;
pub use self::mapper::Mapper;
use core::ops::{Deref, DerefMut};

pub struct ActivePageTable {
    mapper: Mapper,
}

impl Deref for ActivePageTable {
    type Target = Mapper;

    fn deref(&self) -> &Mapper {
        &self.mapper
    }
}

impl DerefMut for ActivePageTable {
    fn deref_mut(&mut self) -> &mut Mapper {
        &mut self.mapper
    }
}

impl ActivePageTable{

    unsafe fn new() -> ActivePageTable {
        ActivePageTable {
            mapper: Mapper::new(),
        }
    }

    pub fn with<F>(&mut self,
                   table: &mut InactivePageTable,
                   temporary_page: &mut temp_page::TemporaryPage, // new
                   f: F)
                    where F: FnOnce(&mut Mapper) {
        use x86_64::instructions::tlb;
        use x86_64::registers::control_regs;

        {

            let backup = Frame::containing_address(
                control_regs::cr3().0 as usize);


        // map temporary_page to current p4 table
            let p4_table = temporary_page.map_table_frame(backup.clone(), self);


        // overwrite recursive mapping
            self.p4_mut()[511].set(table.p4_frame.clone(), PRESENT | WRITABLE);
            tlb::flush_all();

        // execute f in the new context
            f(self);

        // restore recursive mapping to original p4 table
            p4_table[511].set(backup, PRESENT | WRITABLE);
            tlb::flush_all();
        }

        temporary_page.unmap(self);
    }

    pub fn switch(&mut self, new_table: InactivePageTable) -> InactivePageTable {
        use x86_64::PhysicalAddress;
        use x86_64::registers::control_regs;

        let old_table = InactivePageTable {
            p4_frame: Frame::containing_address(
                control_regs::cr3().0 as usize
            ),
        };
        unsafe {
            control_regs::cr3_write(PhysicalAddress(
                new_table.p4_frame.start_address() as u64));
        }
        old_table
    }
}

#[allow(dead_code)]
pub fn test_paging<A>(allocator: &mut A)
    where A: FrameAllocator
{
    use log;
    log("Testing Paging...");
    let mut page_table = unsafe { ActivePageTable::new() };

    let addr = 42 * 512 * 512 * 4096; // 42th P3 entry
    let page = Page::containing_address(addr);
    let frame = allocator.allocate_frame().expect("no more frames");
    println!("None = {:?}, map to {:?}",
         page_table.translate(addr),
         frame);
    page_table.map_to(page, frame, EntryFlags::empty(), allocator);
    println!("Some = {:?}", page_table.translate(addr));
    println!("next free frame: {:?}", allocator.allocate_frame());
}

pub struct InactivePageTable {
    p4_frame : Frame,
}

impl InactivePageTable {
    pub fn new(frame: Frame,
            active_table: &mut ActivePageTable,
            temp_page: &mut TemporaryPage) 
        -> InactivePageTable {
        {
            let table = temp_page.map_table_frame(frame.clone(),
                active_table);
            // now we are able to zero the table
            table.zero();
            // set up recursive mapping for the table
            table[511].set(frame.clone(), PRESENT | WRITABLE);
        }
        temp_page.unmap(active_table);

        InactivePageTable { p4_frame: frame }
    }
}

use multiboot2::BootInformation;
pub fn remap_kernel<A>(allocator: &mut A, boot_info: &BootInformation) 
                    -> ActivePageTable where A: FrameAllocator {
    let mut temporary_page = TemporaryPage::new(Page { number: 0xdea000 },
        allocator); // Random page 


    let mut active_table = unsafe { ActivePageTable::new() };

    let mut new_table = {
        let frame = allocator.allocate_frame().expect("no more frames");
        InactivePageTable::new(frame, &mut active_table, &mut temporary_page)
    };

    active_table.with(&mut new_table, &mut temporary_page, |mapper| {
        let elf_sections_tag = boot_info.elf_sections_tag()
            .expect("Memory map tag required");

        for section in elf_sections_tag.sections() {

            use self::entry::WRITABLE;

            if !section.is_allocated() {
        // section is not loaded to memory
                continue;
            }
            assert!(section.start_address() % PAGE_SIZE == 0,
                "sections need to be page aligned");

            println!("    mapping section at addr: {:#x}, size: {:#x}",
                section.addr, section.size);

            //let flags = WRITABLE; // TODO use real section flags
            let flags = EntryFlags::from_elf_section_flags(section);
            let start_frame = Frame::containing_address(section.start_address());
            let end_frame = Frame::containing_address(section.end_address() - 1);
            for frame in Frame::range_inclusive(start_frame, end_frame) {
                mapper.identity_map(frame, flags, allocator);
            }
        }
        let vga_buffer_frame = Frame::containing_address(0xb8000); 
        mapper.identity_map(vga_buffer_frame, WRITABLE, allocator);

        let multiboot_start = Frame::containing_address(boot_info.start_address());
        let multiboot_end = Frame::containing_address(boot_info.end_address() - 1);
        for frame in Frame::range_inclusive(multiboot_start, multiboot_end) {
            mapper.identity_map(frame, PRESENT, allocator);
        }
    });
    let old_table = active_table.switch(new_table);
    let old_p4_page = Page::containing_address(
        old_table.p4_frame.start_address()
    );
    active_table.unmap(old_p4_page, allocator);
    //println!("guard page at {:#x}", old_p4_page.start_address());
    active_table
}