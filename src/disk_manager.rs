// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::convert::TryInto;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};

use crate::page::{PageID, PAGE_SIZE};

/// A trivial Disk Manager implementation that has all pages in a single large file.
pub struct DiskManager {
    next_page_id: PageID,
    filename: String,
    db_file: File,
}

impl DiskManager {
    // TODO handle file create error
    pub fn new(db_file_name: &str) -> Self {
        Self {
            next_page_id: 0,
            filename: db_file_name.to_owned(),
            db_file: File::create(db_file_name).unwrap(),
        }
    }

    pub fn read_page(&mut self, page: PageID, buf: &mut [u8; PAGE_SIZE]) -> io::Result<()> {
        let offset: u64 = (page * PAGE_SIZE).try_into().unwrap();
        self.db_file.seek(SeekFrom::Start(offset))?;
        self.db_file.read_exact(buf)?;
        Ok(())
    }

    pub fn write_page(&mut self, page: PageID, buf: &[u8; PAGE_SIZE]) -> io::Result<()> {
        let offset: u64 = (page * PAGE_SIZE).try_into().unwrap();
        self.db_file.seek(SeekFrom::Start(offset))?;
        self.db_file.write(buf)?;
        self.db_file.flush()?;
        Ok(())
    }

    pub fn allocate_page(&mut self) -> PageID {
        let id = self.next_page_id;
        self.next_page_id += 1;
        return id;
    }

    pub fn deallocate_page(page: PageID) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {}
}
