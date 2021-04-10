// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

pub trait Replacer {
    /// Removes the next victim page as defined by the replacement strategy.
    /// Returns the page ID of the removed pages if one was found.
    fn remove(&mut self) -> Option<usize>;

    /// Indicates that the page can no longer be victimized until it is unpinned.
    fn pin(&self, usize);

    /// Indicates that the page can now be victimized again.
    fn unpin(&self, usize);

    /// The number of victimizable pages, i.e. remove will succeed iff this if >0.
    fn len(&self) -> usize;
}

struct FIFOReplacer {
}

struct ClockReplacer {
}

struct LRUReplacer {
}

impl Replacer for LRUReplacer {
    fn remove(&mut self) -> Option<usize> {
    }

    fn pin(&mut self, page: usize) {
    }

    fn unpin(&mut self, page: usize) {
    }

    fn len(&mut self) -> usize {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
