// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::convert::TryInto;
use std::fs::{File, OpenOptions};
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
            db_file: OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(db_file_name)
                .unwrap(),
        }
    }

    /// Read the given page of the disk file into the memory buffer.
    pub fn read_page(&mut self, page: PageID, buf: &mut [u8; PAGE_SIZE]) -> io::Result<()> {
        let offset: u64 = (page * PAGE_SIZE).try_into().unwrap();
        self.db_file.seek(SeekFrom::Start(offset))?;
        self.db_file.read_exact(buf)?;
        Ok(())
    }

    /// Write the data from the memory buffer to the given page of the disk file.
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
    fn write_read_page() {
        let mut buf1 = [0u8; PAGE_SIZE];
        let mut buf2 = [0u8; PAGE_SIZE];
        let mut dm = DiskManager::new("xxxxxxx.tmp");

        // read past EOF
        assert!(dm.read_page(0, &mut buf1).is_err());

        for i in 0..10 {
            let s = format!("Page {}", i);
            buf2[..s.len()].copy_from_slice(s.as_bytes());
            assert!(dm.write_page(i, &buf2).is_ok());
            assert!(dm.read_page(i, &mut buf1).is_ok());
            assert_eq!(buf1, buf2);
        }
    }
}
