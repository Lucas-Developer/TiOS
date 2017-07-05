/*  Page table module 
 *  Author: Andrew Jianzhong Liu
 *  All rights reserved
 */

pub mod entries;

use self::entries::Entry;
use core::marker::PhantomData;
use super::super::ENTRY_COUNT;

pub struct PageTable<L: TableLevel> {
    entries: [Entry;ENTRY_COUNT],
    level: PhantomData<L>,
}

// Safety considerations

pub trait TableLevel{}

pub trait HierarchicalTableLevel : TableLevel {
    type NextLevel: TableLevel;
}

pub enum Level4{}
pub enum Level3{}
pub enum Level2{}
pub enum Level1{}

impl TableLevel for Level4{}
impl TableLevel for Level3{}
impl TableLevel for Level2{}
impl TableLevel for Level1{}

impl HierarchicalTableLevel for Level4{
    type NextLevel = Level3;
}
impl HierarchicalTableLevel for Level3{
    type NextLevel = Level2;
}
impl HierarchicalTableLevel for Level2{
    type NextLevel = Level1;
}

use core::ops::{Index, IndexMut};

impl<L> Index<usize> for PageTable<L> where L : TableLevel {
    type Output = Entry;

    fn index(&self, index:usize) -> &Entry {
        &self.entries[index]
    }
}

impl<L> IndexMut<usize> for PageTable<L> where L: TableLevel {
    fn index_mut(&mut self, index: usize) -> &mut Entry {
        &mut self.entries[index]
    }
}

impl <L> PageTable<L> where L: TableLevel{
    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.set_unused()
        }
    }

    pub fn is_empty(&self) -> bool {
        for entry in self.entries.iter() {
            if !entry.is_unused() {
                return false;
            }
        }
        true
    }
}

use super::super::frame::FrameAllocator;
impl<L> PageTable<L> where L: HierarchicalTableLevel {
    fn next_table_address(&self, index: usize) -> Option<usize> {
        use self::entries::*;
        let entry_flags = self[index].flags();
        if entry_flags.contains(PRESENT) && !entry_flags.contains(HUGE_PAGE) {
            let table_address = self as *const _ as usize;
            Some((table_address << 9) | (index << 12))
        }
        else{
            None
        }
    }

    pub fn next_table(&self, index: usize) 
                -> Option<&PageTable<L::NextLevel>> 
                where L: HierarchicalTableLevel{
        self.next_table_address(index)
            .map(|address| unsafe{ &*(address as *const _) })
    }

    pub fn next_table_mut(&mut self, index: usize)
                -> Option<&mut PageTable<L::NextLevel>>
                where L: HierarchicalTableLevel{
        self.next_table_address(index)
            .map(|address| unsafe{ &mut *(address as *mut _)})
    }

    pub fn next_table_create<A>(&mut self, index:usize, allocator: &mut A)
                -> &mut PageTable<L::NextLevel> where A: FrameAllocator {
        use self::entries::*;
        if self.next_table(index).is_none() {
            assert!(!self.entries[index].flags().contains(HUGE_PAGE), "Mapping code does not support huge pages");
            let frame = allocator.allocate_frame().expect("No frames available");
            self.entries[index].set(frame, PRESENT | WRITABLE);
            self.next_table_mut(index).unwrap().zero();
        }
        self.next_table_mut(index).unwrap()
    }
}

use super::super::VirtualAddress;

pub const P4_ADDR : VirtualAddress = 0xffff_ffff_ffff_f000 as VirtualAddress;