// Copyright (C) 2021 Quentin Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::collections::HashMap;
//use std::sync::atomic::AtomicUsize;

pub struct Catalog {
    tables: Vec<TableMetadata>,
    table_names: HashMap<String, usize>,
    //next_table_id: AtomicUsize,
}

impl Catalog {
    pub fn create_table(&mut self, name: &str, schema: &Schema) -> Result<usize, &str> {
        if self.table_names.get(name).is_some() {
            Err("")
        } else {
            let id = self.tables.len() - 1;
            self.tables.push(TableMetadata {
                schema: schema.clone(),
                name: name.to_owned(),
                id,
            });
            Ok(id)
        }
    }

    pub fn get_table(&self, name: &str) -> Option<TableMetadata> {
        match self.table_names.get(name) {
            Some(&id) => Some(self.tables[id].clone()),
            None => None,
        }
    }

    pub fn create_index(&self, name: &str, table: &str) {}

    pub fn get_index(&self, name: &str, table: &str) {}

    pub fn get_table_indices(&self, table: &str) {}
}

#[derive(Clone)]
struct TableMetadata {
    schema: Schema,
    name: String,
    id: usize,
}

#[derive(Clone)]
struct Schema {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
