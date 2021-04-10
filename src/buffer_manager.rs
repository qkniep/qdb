// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::sync::Arc;

use crate::page::*;
use crate::replacer::{ClockReplacer, FrameID, Replacer};

///
pub struct BufferManager<R = ClockReplacer> {
    max_pages: usize,
    pub pages: Vec<Arc<Page>>,
    page_table: HashMap<PageID, FrameID>,
    free_list: VecDeque<PageID>,
    replacer: R,
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
        };
        for i in 0..capacity {
            bm.free_list.push_back(i);
        }
        return bm;
    }

    /// Fetch the requested page, loading it form disk if necessary.
    /// Returns `None` if we failed to allocate the page, i.e. all pages are pinned.
    pub fn fetch_page(&mut self, page: PageID) -> Option<Arc<Page>> {
        // Check if page is already cached
        if let Some(&frame) = self.page_table.get(&page) {
            return Some(self.pages[frame].clone());
        }

        match self.find_free_page() {
            None => None,
            Some(frame) => {
                // TODO actually load page from disk
                let p = Arc::new(Page::new());
                self.pages[frame] = p.clone();
                self.page_table.insert(page, frame);
                self.replacer.pin(frame);
                Some(p)
            }
        }
    }

    /// Allocates a new empty page.
    pub fn allocate_empty_page(&mut self) -> Option<Arc<Page>> {
        match self.find_free_page() {
            None => None,
            Some(frame) => {
                let p = Arc::new(Page::new());
                self.pages[frame] = p.clone();
                self.page_table.insert(p.id, frame);
                self.replacer.pin(frame);
                Some(p)
            }
        }
    }

    ///
    // TODO handle failure b/c pinned already was <= 0
    pub fn unpin_page(&mut self, page: PageID) {
        let frame = self.page_table[&page];
        // FIXME
        //self.pages[frame].pinned -= 1;
        self.replacer.unpin(page);
    }

    /// Flushes the given page to disk.
    // TODO handle page not in page_table
    pub fn flush_page(&mut self, page: PageID) {
        let frame = self.page_table.remove(&page).unwrap();
        self.free_list.push_back(frame);
        if self.pages[frame].dirty {
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
            assert!(mm.allocate_empty_page().is_some());
        }
        assert!(mm.allocate_empty_page().is_none());
    }

    #[test]
    fn pages_free() {
        let mut mm = BufferManager::<ClockReplacer>::new(10);
        for i in 0..10 {
            assert_eq!(mm.pages_free(), 10 - i);
            mm.allocate_empty_page();
        }
        assert_eq!(mm.pages_free(), 0);
        mm.allocate_empty_page();
        assert_eq!(mm.pages_free(), 0);
    }
}
