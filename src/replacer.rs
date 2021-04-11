// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::fmt::Display;

pub type FrameID = usize;

pub trait Replacer {
    /// Initializes a new Replacer with support for a maximum of `capacity` pages.
    /// All frames should initially be unpinned.
    fn new(capacity: usize) -> Self;

    /// Picks the next victim page as defined by the replacement strategy.
    /// Returns the frame ID of the removed page if one was found.
    fn pick_victim(&mut self) -> Option<FrameID>;

    /// Indicates that the given frame can no longer be victimized until it is unpinned.
    fn pin(&mut self, frame: FrameID);

    /// Indicates that the given frame can now be victimized again.
    fn unpin(&mut self, frame: FrameID);

    /// The number of victimizable frames, i.e. `pick_victim()` will succeed iff this is >0.
    fn len(&self) -> usize;
}

#[derive(Clone, Copy, Default)]
struct ClockFrame {
    pinned: bool,
    used: bool,
}

/// Clock (aka "Second Chance") Replacement Strategy.
pub struct ClockReplacer {
    frames: Vec<ClockFrame>,
    hand: FrameID,
    num_unpinned: usize,
}

impl Display for ClockReplacer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, frame) in self.frames.iter().enumerate() {
            write!(f, "{}", i)?;
            if frame.pinned {
                write!(f, "p")?;
            }
            if frame.used {
                write!(f, "*")?;
            }
            write!(f, ", ")?;
        }
        write!(f, " (h={})", self.hand)?;
        Ok(())
    }
}

impl Replacer for ClockReplacer {
    fn new(capacity: usize) -> Self {
        Self {
            frames: vec![ClockFrame::default(); capacity],
            hand: 0,
            num_unpinned: capacity,
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
        let h = self.hand;
        self.hand = (self.hand + 1) % self.frames.len();
        return Some(h);
    }

    fn pin(&mut self, frame: FrameID) {
        self.frames[frame].pinned = true;
        self.num_unpinned -= 1;
    }

    fn unpin(&mut self, frame: FrameID) {
        self.frames[frame].pinned = false;
        self.frames[frame].used = true;
        self.num_unpinned += 1;
    }

    fn len(&self) -> usize {
        self.num_unpinned
    }
}

#[derive(Clone, Default)]
struct LRUFrame {
    prev: FrameID,
    next: FrameID,
}

/// Least Recently Used (LRU) Replacement Strategy.
/// We always pick as victim the page that has gone the longest time without being pinned.
pub struct LRUReplacer {
    frames: Vec<LRUFrame>,
    head: FrameID,
    tail: FrameID,
    num_unpinned: usize,
}

impl Replacer for LRUReplacer {
    fn new(capacity: usize) -> Self {
        let mut r = Self {
            frames: vec![LRUFrame::default(); capacity],
            head: 0,
            tail: capacity - 1,
            num_unpinned: capacity,
        };
        for i in 0..capacity {
            r.frames[i].next = (i + 1) % capacity;
            r.frames[i].prev = (capacity + i - 1) % capacity;
        }
        return r;
    }

    fn pick_victim(&mut self) -> Option<FrameID> {
        if self.len() == 0 {
            return None;
        }

        let h = self.head;
        if self.len() > 1 {
            self.unpin(h);
            self.num_unpinned -= 1;
        }
        Some(h)
    }

    fn pin(&mut self, frame: FrameID) {
        self.remove(frame);
        self.push_back(frame);
        self.num_unpinned -= 1;
    }

    fn unpin(&mut self, frame: FrameID) {
        self.remove(frame);
        self.push_back(frame);
        if self.len() == 0 {
            self.head = frame;
        }
        self.tail = frame;
        self.num_unpinned += 1;
    }

    fn len(&self) -> usize {
        self.num_unpinned
    }
}

/// Utility functions for the underlying data structure.
/// The `frames` vector basically simulates a doubly-linked list.
impl LRUReplacer {
    fn remove(&mut self, frame: FrameID) {
        let p = self.frames[frame].prev;
        let n = self.frames[frame].next;
        self.frames[p].next = n;
        self.frames[n].prev = p;
        if frame == self.head {
            self.head = n;
        } else if frame == self.tail {
            self.tail = p;
        }
    }

    fn push_back(&mut self, frame: FrameID) {
        let t = self.tail;
        let n = self.frames[t].next;
        self.frames[t].next = frame;
        self.frames[n].prev = frame;
        self.frames[frame].prev = t;
        self.frames[frame].next = n;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clock_basic() {
        let mut r = ClockReplacer::new(3);
        for _ in 0..5 {
            assert!(r.pick_victim().is_some());
        }
        r.pin(0);
        for _ in 0..5 {
            let v = r.pick_victim();
            assert!(v.is_some());
            assert_ne!(v, Some(0));
        }
        r.pin(1);
        assert_eq!(r.pick_victim(), Some(2));
        assert_eq!(r.pick_victim(), Some(2));
        r.pin(2);
        assert_eq!(r.pick_victim(), None);
        assert_eq!(r.pick_victim(), None);
        r.unpin(0);
        assert_eq!(r.pick_victim(), Some(0));
        assert_eq!(r.pick_victim(), Some(0));
    }

    #[test]
    fn clock_order() {
        let mut r = ClockReplacer::new(3);
        println!("{}", r);
        assert_eq!(r.pick_victim(), Some(0));
        println!("{}", r);
        use_page(&mut r, 0);
        println!("{}", r);
        assert_eq!(r.pick_victim(), Some(1));
        println!("{}", r);
        use_page(&mut r, 1);
        println!("{}", r);
        use_page(&mut r, 0);
        println!("{}", r);
        assert_eq!(r.pick_victim(), Some(2));
        println!("{}", r);
        use_page(&mut r, 2);
        println!("{}", r);
        assert_eq!(r.pick_victim(), Some(0));
        use_page(&mut r, 0);
        assert_eq!(r.pick_victim(), Some(1));
        use_page(&mut r, 1);
        assert_eq!(r.pick_victim(), Some(2));
        use_page(&mut r, 2);
        use_page(&mut r, 0);
        assert_eq!(r.pick_victim(), Some(0));
        use_page(&mut r, 0);
        use_page(&mut r, 1);
        assert_eq!(r.pick_victim(), Some(2));
        use_page(&mut r, 0);
    }

    #[test]
    fn lru_basic() {
        let mut r = LRUReplacer::new(3);
        for _ in 0..5 {
            assert!(r.pick_victim().is_some());
        }
        r.pin(0);
        for _ in 0..5 {
            let v = r.pick_victim();
            assert!(v.is_some());
            assert_ne!(v, Some(0));
        }
        r.pin(1);
        assert_eq!(r.pick_victim(), Some(2));
        assert_eq!(r.pick_victim(), Some(2));
        r.pin(2);
        assert_eq!(r.pick_victim(), None);
        assert_eq!(r.pick_victim(), None);
        r.unpin(0);
        assert_eq!(r.pick_victim(), Some(0));
        assert_eq!(r.pick_victim(), Some(0));
    }

    #[test]
    fn lru_order() {
        let mut r = LRUReplacer::new(3);
        assert_eq!(r.pick_victim(), Some(0));
        use_page(&mut r, 0);
        assert_eq!(r.pick_victim(), Some(1));
        use_page(&mut r, 1);
        use_page(&mut r, 0);
        assert_eq!(r.pick_victim(), Some(2));
        use_page(&mut r, 2);
        assert_eq!(r.pick_victim(), Some(1));
        use_page(&mut r, 1);
        use_page(&mut r, 0);
        assert_eq!(r.pick_victim(), Some(2));
        use_page(&mut r, 2);
        use_page(&mut r, 1);
        assert_eq!(r.pick_victim(), Some(0));
        use_page(&mut r, 0);
        assert_eq!(r.pick_victim(), Some(2));
        use_page(&mut r, 2);
        use_page(&mut r, 1);
        use_page(&mut r, 2);
    }

    fn use_page<R: Replacer>(r: &mut R, frame: FrameID) {
        r.pin(frame);
        r.unpin(frame);
    }
}
