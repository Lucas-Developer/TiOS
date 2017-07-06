/*  Page table module 
 *  Author: Andrew Jianzhong Liu
 *  All rights reserved
 */
 
use multiboot2::ElfSection;
use super::super::PhysicalAddress;
use super::super::frame::Frame;

pub struct Entry(u64);

bitflags! {
    flags EntryFlags: u64 {
        const PRESENT =         1 << 0,
        const WRITABLE =        1 << 1,
        const USER_ACCESSIBLE = 1 << 2,
        const WRITE_THROUGH =   1 << 3,
        const NO_CACHE =        1 << 4,
        const ACCESSED =        1 << 5,
        const DIRTY =           1 << 6,
        const HUGE_PAGE =       1 << 7,
        const GLOBAL =          1 << 8,
        const NO_EXECUTE =      1 << 63,
    }
}

impl EntryFlags {
    pub fn from_elf(section: &ElfSection) -> EntryFlags {
        use multiboot2::{ELF_SECTION_ALLOCATED, ELF_SECTION_WRITABLE,
            ELF_SECTION_EXECUTABLE};
        let mut flags = EntryFlags::empty();
        if section.flags().contains(ELF_SECTION_ALLOCATED) {
            flags = flags | PRESENT;
        }
        if section.flags().contains(ELF_SECTION_WRITABLE) {
            flags = flags | WRITABLE;
        }
        if !section.flags().contains(ELF_SECTION_EXECUTABLE) {
            flags = flags | NO_EXECUTE;
        }
        flags
    }
}

impl Entry {
    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }
    pub fn set_unused(&mut self) {
        self.0 = 0;
    }
    pub fn flags(&self) -> EntryFlags {
        EntryFlags::from_bits_truncate(self.0)
    }
    pub fn pointed_frame(&self) -> Option<Frame> {
        if self.flags().contains(PRESENT) {
            Some(Frame::from((self.0 as usize & 0x000fffff_fffff000) 
                as PhysicalAddress))
        }
        else{
            None
        }
    }
    pub fn set(&mut self, frame: Frame, flags: EntryFlags) {
        assert_eq!(frame.start_address() & 0xfff00000_00000fff, 0, "Invalid frame starting address {:x}!", frame.start_address());
        self.0 = frame.start_address() as u64 | flags.bits();
    }
}