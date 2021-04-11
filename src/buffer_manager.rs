// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::sync::Arc;

use crate::disk_manager::DiskManager;
use crate::page::*;
use crate::replacer::{ClockReplacer, FrameID, Replacer};

/// The Buffer Manager is responsible for keeping pages in memory, keeping track of pinned pages.
/// It interacts with the Disk Manager to retrieve these pages from disk and write them back.
pub struct BufferManager<R = ClockReplacer> {
    max_pages: usize,
    pub pages: Vec<Arc<Page>>,
    page_table: HashMap<PageID, FrameID>,
    free_list: VecDeque<PageID>,
    replacer: R,
    disk_manager: DiskManager,
}

impl Display for BufferManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BufferManager({}/{} pages free)",
            self.pages_free(),
            self.max_pages
        )
    }
}

impl<R: Replacer> BufferManager<R> {
    ///
    pub fn new(capacity: usize) -> BufferManager<R> {
        let mut bm = BufferManager {
            max_pages: capacity,
            pages: vec![Arc::new(Page::default()); capacity],
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
    pub fn fetch_page(&mut self, page: PageID) -> Option<Arc<Page>> {
        // Check if requested page is already cached
        if let Some(&frame) = self.page_table.get(&page) {
            return Some(self.pages[frame].clone());
        }

        match self.find_free_page() {
            None => None,
            Some(frame) => {
                // TODO actually load page from disk
                let p = Arc::new(Page::new(page));
                self.pages[frame] = p.clone();
                self.page_table.insert(page, frame);
                self.replacer.pin(frame);
                Some(p)
            }
        }
    }

    /// Allocates a new empty page.
    /// This page is pinned immediately.
    pub fn new_page(&mut self) -> Option<Arc<Page>> {
        match self.find_free_page() {
            None => None,
            Some(frame) => {
                let p_id = self.disk_manager.allocate_page();
                let p = Arc::new(Page::new(p_id));
                self.pages[frame] = p.clone();
                self.page_table.insert(p_id, frame);
                self.replacer.pin(frame);
                Some(p)
            }
        }
    }

    ///
    // TODO handle failure b/c pin_count already was <= 0
    pub fn unpin_page(&mut self, page: PageID) {
        let frame = self.page_table[&page];
        // FIXME
        //self.pages[frame].pin_count -= 1;
        self.replacer.unpin(page);
    }

    /// Flushes the given page to disk.
    // TODO handle page not in page_table
    pub fn flush_page(&mut self, page: PageID) {
        let frame = self.page_table.remove(&page).unwrap();
        self.free_list.push_back(frame);
        if self.pages[frame].dirty {
            //self.disk_manager.write_page();
            self.pages[frame].write_to_disk("out");
        }
    }

    ///
    pub fn delete_page(&mut self, page: PageID) {
        let frame = self.page_table[&page];
    }

    pub fn pages_free(&self) -> usize {
        self.free_list.len()
    }

    pub fn bytes_free(&self) -> usize {
        self.pages_free() * PAGE_SIZE
    }

    ///
    fn find_free_page(&mut self) -> Option<PageID> {
        if self.pages_free() == 0 {
            if let Some(frame) = self.replacer.pick_victim() {
                self.flush_page(self.pages[frame].id);
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

    #[test]
    fn allocate_pages() {
        let mut mm = BufferManager::<ClockReplacer>::new(10);
        for _ in 0..10 {
            assert!(mm.new_page().is_some());
        }
        assert!(mm.new_page().is_none());
    }

    #[test]
    fn pages_free() {
        let mut mm = BufferManager::<ClockReplacer>::new(10);
        for i in 0..10 {
            assert_eq!(mm.pages_free(), 10 - i);
            mm.new_page();
        }
        assert_eq!(mm.pages_free(), 0);
        mm.new_page();
        assert_eq!(mm.pages_free(), 0);
    }
}
