// Copyright (C) 2021 qkniep <qkniep@qk-huawei>
// Distributed under terms of the MIT license.

use crate::block::Block;
use crate::buffer_manager::BufferManager;

pub fn join(mm: &mut BufferManager) {
    let output_block = mm.allocate_empty_block();

    // TODO always use smaller relation as outer
}

fn join_blocks(b1: &Block, b2: &Block, leftAttr: usize, rightAttr: usize, out: &mut Block) {}

fn join_tuples() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
