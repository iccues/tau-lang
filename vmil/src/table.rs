use std::collections::HashMap;

use crate::var::Value;

pub struct Table {
    pub value: Option<Value>,
    table: HashMap<String, Table>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            value: None,
            table: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: Table) {
        self.table.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&Table> {
        self.table.get(key)
    }
}
