pub mod byte_vec;
pub mod symbol_table;

use std::cell::{RefCell, RefMut};

use byte_vec::ByteVec;
use symbol_table::{SymbolTable, SymbolTableRef};

pub type GlobalRc = std::rc::Rc<Global>;

pub struct Global {
    symbol_table: SymbolTableRef,
    byte_vec: RefCell<ByteVec>,
}

impl Global {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new_ref(),
            byte_vec: RefCell::new(ByteVec::new()),
        }
    }

    pub fn new_rc() -> GlobalRc {
        std::rc::Rc::new(Self::new())
    }

    pub fn symbol_table(&self) -> &SymbolTableRef {
        &self.symbol_table
    }

    pub fn byte_vec(&self) -> RefMut<ByteVec> {
        self.byte_vec.borrow_mut()
    }
}
