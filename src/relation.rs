// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

use crate::block::{Block, BLOCK_SIZE};
use crate::buffer_manager::BufferManager;
use crate::table_scan::TableScanner;

pub struct Relation<'a> {
    mm: &'a mut BufferManager,
    pub file: String,
}

impl<'a> Relation<'a> {
    pub fn new(mm: &'a mut BufferManager, file: &str) -> Relation<'a> {
        return Relation {
            mm,
            file: file.to_owned(),
        };
    }

    /*pub fn scan(&'a self) -> TableScanner<'a> {
        return TableScanner::new(self, self.mm);
    }*/

    /// Loads the block given by index from this relations disk file.
    pub fn get_block(&'a self, block: usize) -> io::Result<Block> {
        let mut file = File::open(&self.file).unwrap();
        let size = file.seek(SeekFrom::End(0))?;
        file.seek(SeekFrom::Start((block * BLOCK_SIZE) as u64))?;
        let mut b = Block::new();
        b.used_space = file.read(&mut b.data)?;
        return Ok(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut mm = BufferManager::new(10);
        let r = Relation::new(&mut mm, "movies.txt");
        for i in 0..3 {
            let b = r.get_block(i).unwrap();
            assert!(b.used_space > 0);
        }
    }
}
