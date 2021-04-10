// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

mod block;
mod btree;
mod buffer_manager;
mod extensible_hash;
mod external_sort;
mod nested_loop_join;
mod relation;
mod replacer;
mod table_scan;

use buffer_manager::BufferManager;

fn main() {
    let mm = BufferManager::new(1000);
    println!("{}", mm);
}
