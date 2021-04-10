// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::collections::{HashMap, VecDeque};

type FrameID = usize;

pub trait Replacer {
    /// Initializes a new Replacer with support for a maximum of `num_pages` pages.
    /// All pages should initially be unpinned.
    fn new(num_pages: usize) -> Self;

    /// Picks the next victim page as defined by the replacement strategy.
    /// Returns the page ID of the removed pages if one was found.
    fn pick_victim(&mut self) -> Option<FrameID>;

    /// Indicates that the page can no longer be victimized until it is unpinned.
    fn pin(&mut self, page: FrameID);

    /// Indicates that the page can now be victimized again.
    fn unpin(&mut self, page: FrameID);

    /// The number of victimizable pages, i.e. remove will succeed iff this if >0.
    fn len(&self) -> usize;
}

#[derive(Clone, Copy, Default)]
struct ClockFrame {
    pinned: bool,
    used: bool,
}

/// Clock (aka "Second Chance") Replacement Strategy
pub struct ClockReplacer {
    frames: Vec<ClockFrame>,
    hand: FrameID,
    num_unpinned: usize,
}

impl Replacer for ClockReplacer {
    fn new(num_pages: usize) -> Self {
        Self {
            frames: vec![ClockFrame::default(); num_pages],
            hand: 0,
            num_unpinned: num_pages,
        }
    }

    fn pick_victim(&mut self) -> Option<FrameID> {
        if self.len() == 0 {
            return None;
        }

        while self.frames[self.hand].pinned || self.frames[self.hand].used {
            self.frames[self.hand].used = false;
            self.hand = (self.hand + 1) % self.frames.len();
        }
        return Some(self.hand);
    }

    fn pin(&mut self, page: FrameID) {
        self.frames[page].pinned = true;
        self.num_unpinned -= 1;
    }

    fn unpin(&mut self, page: FrameID) {
        self.frames[page].pinned = false;
        self.frames[page].used = true;
        self.num_unpinned += 1;
    }

    fn len(&self) -> usize {
        self.num_unpinned
    }
}

struct LRUFrame {
    id: FrameID,
}

/// Least Recently Used Replacement Strategy
pub struct LRUReplacer {
    frames: VecDeque<LRUFrame>,
    pos: HashMap<FrameID, usize>,
}

impl Replacer for LRUReplacer {
    fn new(num_pages: usize) -> Self {
        Self {
            frames: VecDeque::with_capacity(num_pages),
            pos: HashMap::with_capacity(num_pages),
        }
    }

    fn pick_victim(&mut self) -> Option<FrameID> {
        if let Some(page) = self.frames.pop_front() {
            self.pos.remove(&page.id);
            Some(page.id)
        } else {
            None
        }
    }

    fn pin(&mut self, page: FrameID) {
        self.frames.remove(self.pos[&page]);
        self.pos.remove(&page);
    }

    fn unpin(&mut self, page: FrameID) {
        self.frames.push_back(LRUFrame { id: page });
    }

    fn len(&self) -> usize {
        self.frames.len()
    }
}

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

    #[test]
    fn lru() {
        let mut r = LRUReplacer::new(3);
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
