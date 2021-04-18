// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};

use crate::disk_manager::DiskManager;
use crate::page::*;
use crate::replacer::{ClockReplacer, FrameID, Replacer};

/// The Buffer Manager is responsible for keeping pages in memory, keeping track of pinned pages.
/// It interacts with the Disk Manager to retrieve these pages from disk and write them back.
/// All other parts of the DBMS get their memory buffers from here.
pub struct BufferManager<R = ClockReplacer>
where
    R: Replacer,
{
    max_pages: usize,
    pub pages: Vec<Arc<RwLock<Page>>>,
    page_table: HashMap<PageID, FrameID>,
    free_list: VecDeque<PageID>,
    replacer: R,
    disk_manager: DiskManager,
}

impl<R: Replacer> BufferManager<R> {
    /// Initiate a new Buffer Manager.
    pub fn new(capacity: usize) -> BufferManager<R> {
        let mut bm = BufferManager {
            max_pages: capacity,
            pages: vec![Arc::new(RwLock::new(Page::default())); capacity],
            page_table: HashMap::with_capacity(capacity),
            free_list: VecDeque::with_capacity(capacity),
            replacer: R::new(capacity),
            disk_manager: DiskManager::new("xxxx.tmp"),
        };
        for i in 0..capacity {
            bm.free_list.push_back(i);
        }
        return bm;
    }

    /// Fetch the requested page, loading it form disk if necessary.
    /// Returns `None` if we failed to allocate the page, i.e. all pages are pinned.
    // TODO don't panic
    pub fn fetch_page(&mut self, page: PageID) -> Option<Arc<RwLock<Page>>> {
        // Check if requested page is already cached
        if let Some(&frame) = self.page_table.get(&page) {
            return Some(self.pages[frame].clone());
        }

        match self.find_free_page() {
            None => None,
            Some(frame) => {
                let p = Arc::new(RwLock::new(Page::new(page)));
                if self
                    .disk_manager
                    .read_page(page, &mut p.write().unwrap().data)
                    .is_err()
                {
                    panic!("failed to read page from disk");
                }
                self.pages[frame] = p.clone();
                self.page_table.insert(page, frame);
                self.replacer.pin(frame);
                Some(p)
            }
        }
    }

    /// Allocates a new empty page.
    /// This page is pinned immediately.
    pub fn new_page(&mut self) -> Option<Arc<RwLock<Page>>> {
        match self.find_free_page() {
            None => None,
            Some(frame) => {
                let p_id = self.disk_manager.allocate_page();
                let p = Arc::new(RwLock::new(Page::new(p_id)));
                self.pages[frame] = p.clone();
                self.page_table.insert(p_id, frame);
                self.replacer.pin(frame);
                Some(p)
            }
        }
    }

    ///
    // TODO don't panic
    pub fn unpin_page(&mut self, page: PageID, dirty: bool) {
        let frame = self.page_table[&page];
        let mut p = self.pages[frame].write().unwrap();

        // Fail if not pinned
        if p.pin_count <= 0 {
            panic!("tried to unpin page that was not pinned");
        }

        if dirty {
            p.dirty = true;
        }
        p.pin_count -= 1;
        self.replacer.unpin(frame);
    }

    /// Flushes the given page to disk.
    // TODO handle page not in page_table
    // TODO don't panic
    pub fn flush_page(&mut self, page: PageID) {
        let frame = self.page_table.remove(&page).unwrap();
        let p = self.pages[frame].read().unwrap();
        self.free_list.push_back(frame);
        if p.dirty {
            if self.disk_manager.write_page(page, &p.data).is_err() {
                panic!("failed to write page to disk");
            }
        }
    }

    ///
    // TODO
    pub fn delete_page(&mut self, page: PageID) {
        let frame = self.page_table[&page];
    }

    /// Number of pages currently free (i.e. not used at all, pinned or unpinned).
    pub fn pages_free(&self) -> usize {
        self.free_list.len()
    }

    /// Finds a free page from the free list.
    /// Frees an unpinned page first if necessary.
    fn find_free_page(&mut self) -> Option<PageID> {
        if self.pages_free() == 0 {
            if let Some(frame) = self.replacer.pick_victim() {
                let l = self.pages[frame].clone();
                let p = l.read().unwrap();
                self.flush_page(p.id);
            } else {
                return None;
            }
        }

        Some(self.free_list.pop_front().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CAPACITY: usize = 10;

    #[test]
    fn allocate_pages() {
        let mut mm = BufferManager::<ClockReplacer>::new(CAPACITY);
        for i in 0..CAPACITY {
            assert_eq!(mm.pages_free(), CAPACITY - i);
            assert!(mm.new_page().is_some());
        }
        for _ in 0..CAPACITY {
            assert_eq!(mm.pages_free(), 0);
            assert!(mm.new_page().is_none());
        }
        assert_eq!(mm.pages_free(), 0);
    }

    #[test]
    fn unpin_pages() {
        let mut mm = BufferManager::<ClockReplacer>::new(CAPACITY);
        for i in 0..CAPACITY {
            assert_eq!(mm.pages_free(), CAPACITY - i);
            assert!(mm.new_page().is_some());
        }
        for i in 0..CAPACITY {
            mm.unpin_page(i, false);
        }
        for _ in 0..CAPACITY - 1 {
            assert!(mm.new_page().is_some());
        }
        assert!(mm.fetch_page(0).is_some());
        mm.unpin_page(0, false);
        assert!(mm.new_page().is_some());
        assert!(mm.fetch_page(0).is_none());
    }

    #[test]
    fn write_and_read() {
        let mut mm = BufferManager::<ClockReplacer>::new(CAPACITY);
        let p_opt = mm.new_page();
        assert!(p_opt.is_some());
        let p = p_opt.unwrap();
        // Write something into this page
        let s = "Hello".as_bytes();
        p.write().unwrap().data[..s.len()].clone_from_slice(s);
        // Read it back from cache
        assert_eq!(p.read().unwrap().data[..5], *"Hello".as_bytes());
        // Force the page out of cache
        mm.unpin_page(0, true);
        for _ in 0..CAPACITY {
            assert!(mm.new_page().is_some());
        }
        for i in 0..CAPACITY {
            mm.unpin_page(i + 1, false);
        }
        // Read page from disk and compare with written value
        let p_opt = mm.fetch_page(0);
        assert!(p_opt.is_some());
        let p = p_opt.unwrap();
        assert_eq!(p.read().unwrap().data[..5], *"Hello".as_bytes());
    }
}
