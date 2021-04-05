// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

pub const BLOCK_SIZE: usize = 4096;

pub struct Block {
    pub data: [u8; BLOCK_SIZE],
    pub used_space: usize,
    pub pinned: usize,
}

impl Block {
    pub fn new() -> Block {
        return Block {
            data: [0; BLOCK_SIZE],
            used_space: 0,
            pinned: 0,
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
