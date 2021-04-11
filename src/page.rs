// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

pub const PAGE_SIZE: usize = 4096;

pub type PageID = usize;

pub struct Page {
    pub id: PageID,
    pub dirty: bool,
    pub used_space: usize,
    pub pin_count: usize,
    pub data: [u8; PAGE_SIZE],
}

impl Default for Page {
    fn default() -> Self {
        Self {
            id: 0,
            dirty: false,
            used_space: 0,
            pin_count: 0,
            data: [0; PAGE_SIZE],
        }
    }
}

impl Page {
    pub fn new(id: PageID) -> Self {
        Self {
            id,
            dirty: false,
            used_space: 0,
            pin_count: 1,
            data: [0; PAGE_SIZE],
        }
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
