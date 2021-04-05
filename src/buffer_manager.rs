// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::collections::{HashMap, VecDeque};
use std::fmt::Display;
use std::sync::Arc;

use crate::block::*;

///
pub struct BufferManager {
    max_blocks: usize,
    pub blocks: HashMap<u64, Arc<Block>>,
    pub unfixed: VecDeque<u64>,
}

impl Display for BufferManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BufferManager({}/{} pages free)",
            self.blocks_free(),
            self.max_blocks
        )
    }
}

impl BufferManager {
    pub fn new(capacity: usize) -> BufferManager {
        return BufferManager {
            max_blocks: capacity,
            blocks: HashMap::new(),
            unfixed: VecDeque::new(),
        };
    }

    pub fn get_block(&mut self, block: u64) -> Arc<Block> {
        if let Some(b) = self.blocks.get(&block) {
            return b.clone();
        }

        // TODO handle the case where unfixed is empty
        if self.blocks_free() == 0 {
            let b_id = self.unfixed.pop_front().unwrap();
            let b = self.blocks.remove(&b_id).unwrap();
            b.write_to_disk("out");
        }

        // TODO actually load block from disk
        let b = Arc::new(Block::new());
        self.blocks.insert(block, b.clone());
        return b;
    }

    pub fn allocate_empty_block(&mut self) -> Result<Block, ()> {
        if self.blocks_free() == 0 {
            return Err(());
        }
        //self.blocks.push(Block::new());
        return Ok(Block::new());
    }

    pub fn unpin_block() {}

    pub fn blocks_free(&self) -> usize {
        return self.max_blocks - self.blocks.len();
    }

    pub fn memory_free(&self) -> usize {
        return self.blocks_free() * BLOCK_SIZE;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocate_blocks() {
        let mut mm = BufferManager::new(10);
        for _ in 0..10 {
            assert!(mm.allocate_empty_block().is_ok());
        }
        assert!(mm.allocate_empty_block().is_err());
    }

    #[test]
    fn blocks_free() {
        let mut mm = BufferManager::new(10);
        for i in 0..10 {
            assert_eq!(mm.blocks_free(), 10 - i);
            mm.allocate_empty_block();
        }
        assert_eq!(mm.blocks_free(), 0);
        mm.allocate_empty_block();
        assert_eq!(mm.blocks_free(), 0);
    }
}
