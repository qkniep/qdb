// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use crate::buffer_manager::BufferManager;
use crate::page::Page;

pub fn join(mm: &mut BufferManager) {
    let output_page = mm.new_page();

    // TODO always use smaller relation as outer
}

fn join_pages(p1: &Page, p2: &Page, left_attr: usize, right_attr: usize, out: &mut Page) {}

fn join_tuples() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
