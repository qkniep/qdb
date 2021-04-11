// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

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

/// Clock (aka "Second Chance") Replacement Strategy
pub struct ClockReplacer {
    frames: Vec<ClockFrame>,
    hand: FrameID,
    num_unpinned: usize,
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
        return Some(self.hand);
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

/// Least Recently Used Replacement Strategy
pub struct LRUReplacer {
    frames: Vec<LRUFrame>,
    head: FrameID,
    tail: FrameID,
    len: usize,
}

impl Replacer for LRUReplacer {
    fn new(capacity: usize) -> Self {
        let mut r = Self {
            frames: vec![LRUFrame::default(); capacity],
            head: 0,
            tail: capacity - 1,
            len: capacity,
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

        // remove from front
        let h = self.head;
        self.head = self.frames[h].next;
        self.len -= 1;

        self.unpin(h);
        Some(h)
    }

    fn pin(&mut self, frame: FrameID) {
        // remove frame
        let p = self.frames[frame].prev;
        let n = self.frames[frame].next;
        self.frames[p].next = n;
        self.frames[n].prev = p;
        if frame == self.head {
            self.head = n;
        } else if frame == self.tail {
            self.tail = p;
        }

        self.push_back(frame);
        self.len -= 1;
    }

    fn unpin(&mut self, frame: FrameID) {
        self.push_back(frame);
        self.tail = frame;
        self.len += 1;
    }

    fn len(&self) -> usize {
        self.len
    }
}

impl LRUReplacer {
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
    fn clock() {
        let mut r = ClockReplacer::new(3);
        for _ in 0..5 {
            assert!(r.pick_victim().is_some());
        }
        r.pin(0);
        for _ in 0..5 {
            assert!(r.pick_victim().is_some());
        }
        r.pin(1);
        for _ in 0..5 {
            assert!(r.pick_victim().is_some());
        }
        r.pin(2);
        for _ in 0..5 {
            assert!(r.pick_victim().is_none());
        }
        r.unpin(0);
        for _ in 0..5 {
            assert!(r.pick_victim().is_some());
        }
    }

    #[test]
    fn lru() {
        let mut r = LRUReplacer::new(3);
        for i in 0..5 {
            println!("{}", i);
            assert!(r.pick_victim().is_some());
        }
        r.pin(0);
        for _ in 0..5 {
            assert!(r.pick_victim().is_some());
        }
        r.pin(1);
        for _ in 0..5 {
            assert!(r.pick_victim().is_some());
        }
        r.pin(2);
        for _ in 0..5 {
            assert!(r.pick_victim().is_none());
        }
        r.unpin(0);
        for _ in 0..5 {
            assert!(r.pick_victim().is_some());
        }
    }
}
