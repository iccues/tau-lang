use std::vec::Vec;

use super::*;

#[derive(Debug)]
pub struct VisualMemroy {
    super_memory: *mut dyn Memory,
    page_table: Vec<Option<usize>>,
    len: usize,
}

impl VisualMemroy {
    pub fn new(super_memory: *mut dyn Memory) -> Self {
        VisualMemroy {
            super_memory,
            len: 0,
            page_table: Vec::with_capacity(16),
        }
    }

    fn parent(&self) -> &mut dyn Memory {
        unsafe { &mut *self.super_memory }
    }
}

impl Memory for VisualMemroy {
    fn get_block(&mut self, index: usize) -> Option<&mut Block> {
        self.parent().get_block(self.page_table[index]?)
    }

    fn len(&self) -> usize {
        self.len
    }

    fn set_page(&mut self, index: usize, page: usize) {
        if self.page_table.len() <= index {
            self.page_table.resize(index + 1, None);
        }
        self.page_table[index] = Some(page);
    }

    fn del_page(&mut self, index: usize) {
        self.page_table[index] = None;
    }


    fn as_cursor(&mut self) -> MemoryCursor {
        MemoryCursor::new(self)
    }
}

impl MemoryIndex for VisualMemroy {
    fn check_index(&self, index: usize) -> bool {
        index <= self.len()
        // TODO
    }

    fn index(&self, index: usize) -> &u8 {
        let i = self.page_table[index / BLOCK_SIZE].unwrap();
        &self.parent().get_block(i).unwrap()[index % BLOCK_SIZE]
    }

    fn index_mut(&mut self, index: usize) -> &mut u8 {
        let i = self.page_table[index / BLOCK_SIZE].unwrap();
        &mut self.parent().get_block(i).unwrap()[index % BLOCK_SIZE]
    }
}