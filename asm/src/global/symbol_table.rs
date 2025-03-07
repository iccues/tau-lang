use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub type SymbolTableRef = Rc<SymbolTable>;

#[derive(Debug)]
pub struct SymbolTable {
    symbol_table: RefCell<HashMap<String, usize>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbol_table: RefCell::new(HashMap::new()),
        }
    }

    pub fn new_ref() -> SymbolTableRef {
        Rc::new(Self::new())
    }

    pub fn insert(&self, symbol: &str, address: usize) {
        self.symbol_table
            .borrow_mut()
            .insert(symbol.to_string(), address);
    }

    pub fn get(&self, symbol: &str) -> Option<usize> {
        self.symbol_table.borrow().get(symbol).cloned()
    }

    pub fn entry(self: &Rc<Self>, symbol: &str) -> SymbolTableEntry {
        SymbolTableEntry {
            symbol_table: self.clone(),
            symbol: symbol.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SymbolTableEntry {
    symbol_table: Rc<SymbolTable>,
    symbol: String,
}

impl SymbolTableEntry {
    pub fn get(&self) -> Option<usize> {
        self.symbol_table.get(&self.symbol)
    }

    pub fn set(&self, address: usize) {
        self.symbol_table.insert(&self.symbol, address)
    }
}
