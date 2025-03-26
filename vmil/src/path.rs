use crate::table::Table;

#[derive(Debug)]
pub struct Path {
    names: Vec<String>,
}

impl Path {
    pub fn new(names: Vec<String>) -> Path {
        Path { names }
    }
    
    pub fn search<'a>(&'a self, table: &'a Table) -> Option<&'a Table> {
        let mut table = table;
        for name in &self.names {
            if let Some(t) = table.get(name) {
                table = t;
            } else {
                return None;
            }
        }
        Some(table)
    }
}