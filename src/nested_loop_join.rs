// Copyright (C) 2021 qkniep <qkniep@qk-huawei>
// Distributed under terms of the MIT license.

use crate::buffer_manager::BufferManager;
use crate::page::Page;

pub fn join(mm: &mut BufferManager) {
    let output_page = mm.allocate_empty_page();

    // TODO always use smaller relation as outer
}

fn join_pages(p1: &Page, p2: &Page, leftAttr: usize, rightAttr: usize, out: &mut Page) {}

fn join_tuples() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
