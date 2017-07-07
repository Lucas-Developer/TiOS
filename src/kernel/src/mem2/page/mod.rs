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

pub struct InnerPageTable {
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

    pub fn map_to<A> (&mut self, page:Page, frame:Frame, flags: EntryFlags, allocator: &mut A) where A: FrameAllocator {

        //println!("Page starting address: {:x}", page.start_address());
        //println!("Page : {:#x}", page.start_address());
        //println!("Page indices: {:#x} {:#x} {:#x} {:#x}", page.p4_index(), page.p3_index(), page.p2_index(), page.p1_index());
        let mut p3 = self.p4_mut().next_table_create(page.p4_index(), allocator);
        let mut p2 = p3.next_table_create(page.p3_index(), allocator);
        let mut p1 = p2.next_table_create(page.p2_index(), allocator);
        assert!(p1[page.p1_index()].is_unused());
        use self::entries::PRESENT;
        p1[page.p1_index()].set(frame, flags | PRESENT);
    }

    pub fn map<A>(&mut self, page:Page, flags: EntryFlags, allocator: &mut A) 
                                                            where A: FrameAllocator {
        let frame = allocator.allocate_frame().expect("No remaining free frames");
        self.map_to(page,frame,flags,allocator);
    }

    pub fn identity_map<A> (&mut self, frame: Frame, flags: EntryFlags, 
                            allocator: &mut A) where A: FrameAllocator {
        let page = Page::from(frame.start_address() as VirtualAddress); // PA = VA
        self.map_to(page,frame,flags,allocator);
    }

    pub fn higher_kernel_map<A> (&mut self, frame: Frame, flags: EntryFlags, 
                                 allocator: &mut A) where A: FrameAllocator {

        let page = Page {
            number: (frame.start_address() + KERNEL_VMA) / PAGE_SIZE,
        };

        //println!("{:#x}", frame.start_address() + KERNEL_VMA);

        println!("Mapping {:?} to {:?}", page, frame);
        self.map_to(page,frame,flags,allocator);
    }

    pub fn unmap<A> (&mut self, page: Page, allocator: &mut A) where A: FrameAllocator {

        assert!(self.translate(page.start_address()).is_some());
        let p1 = self.p4_mut()
                     .next_table_mut(page.p4_index())
                     .and_then(|p3| p3.next_table_mut(page.p3_index()))
                     .and_then(|p2| p2.next_table_mut(page.p2_index()))
                     .expect("mapping code does not support huge pages");
        let frame = p1[page.p1_index()].pointed_frame().unwrap();
        p1[page.p1_index()].set_unused();

        // free empty tables

        if p1.is_empty() {
            // unmap p1 and recurse for p2
        }

        use x86_64::instructions::tlb;
        use x86_64::VirtualAddress;
        tlb::flush(VirtualAddress(page.start_address()));
        allocator.deallocate_frame(frame);
    }
}

impl Deref for ActivePageTable {
    type Target = InnerPageTable;

    fn deref(&self) -> &InnerPageTable {
        &self.inner
    }
}

impl DerefMut for ActivePageTable {
    fn deref_mut(&mut self) -> &mut InnerPageTable {
        &mut self.inner
    }
}

impl ActivePageTable {
    pub unsafe fn new() -> ActivePageTable {
        ActivePageTable {
            inner: InnerPageTable::new()
        }
    }

    pub fn with<F> (&mut self, table: &mut InactivePageTable, 
                    temporary_page: &mut TemporaryPage, f: F) 
                    where F: FnOnce(&mut InnerPageTable){
        use x86_64::instructions::tlb;
        use x86_64::registers::control_regs;

        {
            let backup = Frame::from(control_regs::cr3().0 as PhysicalAddress);
            let p4_table = temporary_page.map_table_frame(backup.clone(), self);
            self.p4_mut()[511].set(table.p4_frame.clone(), PRESENT|WRITABLE);
            tlb::flush_all();
            f(self);
            p4_table[511].set(backup, PRESENT | WRITABLE);
            tlb::flush_all();
        }
        temporary_page.unmap(self);
    }

    pub fn switch(&mut self, new_table: InactivePageTable) -> InactivePageTable {
        use x86_64;
        use x86_64::registers::control_regs;

        let old_table = InactivePageTable{
            p4_frame: Frame::from(control_regs::cr3().0 as PhysicalAddress),
        };

        unsafe{
            control_regs::cr3_write(x86_64::PhysicalAddress(
                (new_table.p4_frame.start_address()) as u64));
        }
        old_table
    }
}

#[derive(Debug)]
pub struct InactivePageTable {
    p4_frame: Frame,
}

impl InactivePageTable {
    fn new(frame: Frame, active_table: &mut ActivePageTable, 
               temp_page: &mut TemporaryPage) -> InactivePageTable {
        {
            let table = temp_page.map_table_frame(frame.clone(), active_table);
            table.zero();
            table[511].set(frame.clone(), PRESENT | WRITABLE);
        }
        temp_page.unmap(active_table);
        InactivePageTable{ p4_frame: frame}
    }
}

#[derive(Debug)]
struct TemporaryPage {
    page: Page,
    allocator: TinyAllocator,
}

impl TemporaryPage {
    pub fn new<A>(page: Page, allocator: &mut A) -> TemporaryPage where A: FrameAllocator {
        TemporaryPage{
            page: page,
            allocator: TinyAllocator::new(allocator),
        }
    }

    pub fn map(&mut self, frame: Frame, active_table: &mut ActivePageTable) -> VirtualAddress {
        use self::entries::WRITABLE;
        assert!(active_table.translate_page(self.page).is_none(), "temp page already mapped.");
        active_table.map_to(self.page, frame, WRITABLE, &mut self.allocator);
        self.page.start_address()
    }

    pub fn unmap(&mut self, active_table: &mut ActivePageTable) {
        active_table.unmap(self.page, &mut self.allocator)
    }

    pub fn map_table_frame(&mut self, frame: Frame, active_table: &mut ActivePageTable) -> &mut PageTable<Level1> {
        unsafe{ &mut *(self.map(frame, active_table) as *mut PageTable<Level1>)}
    }

    pub fn free<A>(&mut self, allocator: &mut A) where A: FrameAllocator{
        self.allocator.free_frames(allocator);
    }
}

#[derive(Debug)]
struct TinyAllocator([Option<Frame>;3]);

impl TinyAllocator{
    fn new<A>(allocator: &mut A) -> TinyAllocator where A: FrameAllocator {
        let mut alloc = || allocator.allocate_frame();
        let frames = [alloc(), alloc(), alloc()];
        TinyAllocator(frames)
    }

    fn free_frames<A>(&mut self, allocator: &mut A) where A: FrameAllocator {
        for frame in &mut self.0 {
            if frame.is_some() {
                let a = frame.take().unwrap();
                allocator.deallocate_frame(a);
            }
        }
    }
}

impl FrameAllocator for TinyAllocator{
    fn allocate_frame(&mut self) -> Option<Frame> {
        for frame in &mut self.0 {
            if frame.is_some(){
                return frame.take()
            }
        }
        None
    }

    fn allocate_contiguous_frames(&mut self, num: usize) -> Option<FrameIter> {
        unimplemented!() // And will not implement
    }

    fn deallocate_frame(&mut self, frame: Frame) {
        for _frame in &mut self.0 {
            if _frame.is_none() {
                *_frame = Some(frame);
                return;
            }
        }
        panic!("Too many frames for tiny allocator");
    }
}


pub fn remap_kernel<FA>(boot_info: &multiboot2::BootInformation, 
                    allocator: &mut FA) -> ActivePageTable
                    where FA: FrameAllocator { // TODO: return active table

    let mut temporary_page = TemporaryPage::new(Page{number: 0xdeadbeaf}, allocator);

    let mut active_table = unsafe{ActivePageTable::new()};

    let mut new_table = {
        let frame = allocator.allocate_frame().expect("No frames available");
        InactivePageTable::new(frame, &mut active_table, &mut temporary_page)
    };

    active_table.with(&mut new_table,&mut temporary_page, |innerpt| {
        let elf_sections_tag = boot_info.elf_sections_tag()
            .expect("Memory map tag required");
        for section in elf_sections_tag.sections() {
            use self::entries::WRITABLE;

            if !section.is_allocated() {
                continue;
            }

            let section_start_vma = section.start_address() as VirtualAddress;
            let section_end_vma = (section.end_address() - 1) as VirtualAddress;

            let section_start_pma = {
                if section_start_vma > KERNEL_VMA {
                    (section_start_vma - KERNEL_VMA) as PhysicalAddress
                }
                else{
                    section_start_vma as PhysicalAddress
                }
            };
            let section_end_pma = {
                if section_end_vma > KERNEL_VMA {
                    (section_end_vma - KERNEL_VMA) as PhysicalAddress
                }
                else{
                    section_end_vma as PhysicalAddress
                }
            };

            //println!("Section range: {:#x} - {:#x}", section_start_pma, section_end_pma);

            assert!(section_start_pma % PAGE_SIZE == 0,
                "sections need to be page aligned");

            //println!("    mapping section at addr: {:#x}, size: {:#x}",
            //    section_start_pma, section.size);

            let flags = EntryFlags::from_elf(section);
            let start_frame = Frame::from(section_start_pma);
            let end_frame = Frame::from(section_end_pma);

            //println!("Section frames: {:?} {:?}", start_frame, end_frame);

            
            for frame in Frame::range_inclusive(start_frame, end_frame) {
                //println!("Frame to map: {:?}", frame);
                innerpt.higher_kernel_map(frame, flags, allocator);
            }
            
        }

        let vga_buffer_frame = Frame::from(0xb8000 as PhysicalAddress); 
        innerpt.higher_kernel_map(vga_buffer_frame, WRITABLE, allocator);

        let multiboot_start = Frame::from(boot_info.start_address() as PhysicalAddress);
        let multiboot_end = Frame::from((boot_info.end_address() - 1) as PhysicalAddress);
        for frame in Frame::range_inclusive(multiboot_start, multiboot_end) {
            innerpt.higher_kernel_map(frame, PRESENT, allocator);
        }
    });
    
    let old_table = active_table.switch(new_table);
    
    let old_p4_page = Page::from(
        (old_table.p4_frame.start_address() + KERNEL_VMA) as VirtualAddress
    );
    active_table.unmap(old_p4_page, allocator);
    temporary_page.free(allocator);

    println!("Finished kernel remapping!");

    active_table
}
