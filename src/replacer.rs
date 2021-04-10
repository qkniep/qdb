// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::collections::VecDeque;

type PageID = usize;

pub trait Replacer {
    fn new(num_pages: usize) -> Self;

    /// Picks the next victim page as defined by the replacement strategy.
    /// Returns the page ID of the removed pages if one was found.
    fn pick_victim(&mut self) -> Option<PageID>;

    /// Indicates that the page can no longer be victimized until it is unpinned.
    fn pin(&mut self, page: PageID);

    /// Indicates that the page can now be victimized again.
    fn unpin(&mut self, page: PageID);

    /// The number of victimizable pages, i.e. remove will succeed iff this if >0.
    fn len(&self) -> usize;
}

#[derive(Clone, Copy, Default)]
struct ClockFrame {
    pinned: bool,
    used: bool,
}

/// Clock (aka "Second Chance") Replacement Strategy
struct ClockReplacer {
    list: Vec<ClockFrame>,
    hand: PageID,
    num_unpinned: usize,
}

impl Replacer for ClockReplacer {
    fn new(num_pages: usize) -> Self {
        Self {
            list: vec![ClockFrame::default(); num_pages],
            hand: 0,
            num_unpinned: num_pages,
        }
    }

    fn pick_victim(&mut self) -> Option<PageID> {
        if self.len() == 0 {
            return None;
        }

        while self.list[self.hand].pinned || self.list[self.hand].used {
            self.list[self.hand].used = false;
            self.hand = (self.hand + 1) % self.list.len();
        }
        return Some(self.hand);
    }

    fn pin(&mut self, page: PageID) {
        self.list[page].pinned = true;
        self.num_unpinned -= 1;
    }

    fn unpin(&mut self, page: PageID) {
        self.list[page].pinned = false;
        self.list[page].used = true;
        self.num_unpinned += 1;
    }

    fn len(&self) -> usize {
        self.num_unpinned
    }
}

/*/// Least Recently Used Replacement Strategy
struct LRUReplacer {
    list: VecDeque<Page>,
}

impl Replacer for LRUReplacer {
    fn new(num_pages: usize) -> Self {
        Self {
            list: VecDeque::with_capacity(num_pages),
        }
    }

    fn pick_victim(&mut self) -> Option<PageID> {
        return None;
    }

    fn pin(&mut self, page: PageID) {}

    fn unpin(&mut self, page: PageID) {}

    fn len(&self) -> usize {
        self.list.len()
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clock() {
        let mut r = ClockReplacer::new(3);
        for _ in 0..5 {
            assert_ne!(r.pick_victim(), None);
        }
        r.pin(0);
        for _ in 0..5 {
            assert_ne!(r.pick_victim(), None);
        }
        r.pin(1);
        for _ in 0..5 {
            assert_ne!(r.pick_victim(), None);
        }
        r.pin(2);
        for _ in 0..5 {
            assert_eq!(r.pick_victim(), None);
        }
        r.unpin(0);
        for _ in 0..5 {
            assert_ne!(r.pick_victim(), None);
        }
    }
}
