// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

pub const PAGE_SIZE: usize = 4096;

pub type PageID = usize;

pub struct Page {
    pub id: PageID,
    pub data: [u8; PAGE_SIZE],
    pub dirty: bool,
    pub used_space: usize,
    pub pinned: usize,
}

impl Default for Page {
    fn default() -> Self {
        return Self {
            id: 0,
            data: [0; PAGE_SIZE],
            dirty: false,
            used_space: 0,
            pinned: 1,
        };
    }
}

impl Page {
    // TODO assign different IDs
    pub fn new() -> Self {
        return Self {
            id: 0,
            data: [0; PAGE_SIZE],
            dirty: false,
            used_space: 0,
            pinned: 1,
        };
    }

    pub fn add_tuple() {}

    pub fn write_to_disk(&self, file: &str) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
