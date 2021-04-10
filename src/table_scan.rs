// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use crate::buffer_manager::BufferManager;
use crate::page::*;
use crate::relation::Relation;

pub struct TableScanner<'a> {
    file: File,
    rel: &'a Relation<'a>,
    mm: &'a mut BufferManager,
}

impl<'a> TableScanner<'a> {
    pub fn new(rel: &'a Relation, mm: &'a mut BufferManager) -> TableScanner<'a> {
        let file = File::open(&rel.file).unwrap();
        return TableScanner { file, rel, mm };
    }
}

impl<'a> Iterator for TableScanner<'a> {
    type Item = Arc<Page>;

    // TODO handle errors & deallocate page if read 0 bytes
    fn next(&mut self) -> Option<Self::Item> {
        /*let mut p = self.mm.allocate_empty_page().unwrap();
        let n = self.file.read(&mut p.data).unwrap();
        match n {
            0 => None,
            default => Some(p),
        }*/
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan() {
        let mut mm = BufferManager::new(20);
        let r = Relation::new(&mut mm, "movies.txt");
        //let scanner = TableScanner::new(&r, &mut mm);
        //for b in scanner {}
    }
}
