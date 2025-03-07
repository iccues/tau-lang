use super::*;

#[derive(Debug)]
pub struct RealMemory {
    len: usize,
    blocks: Vec<Box<Block>>,
}

#[allow(dead_code)]
impl RealMemory {
    pub fn new() -> Self {
        RealMemory {
            len: 0,
            blocks: Vec::with_capacity(16),
        }
    }

    pub fn with_len(len: usize) -> Self {
        RealMemory {
            len,
            blocks: vec![Box::new([0; BLOCK_SIZE]); len.div_ceil(BLOCK_SIZE)],
        }
    }

    pub fn resize(&mut self, new_len: usize) {
        self.len = new_len;
        self.blocks
            .resize(self.len.div_ceil(BLOCK_SIZE), Box::new([0u8; BLOCK_SIZE]));
    }

    pub fn extend(&mut self, len: usize) {
        self.resize(self.len + len);
    }
}

impl Memory for RealMemory {
    fn get_block(&mut self, index: usize) -> Option<&mut Block> {
        self.blocks
            .get_mut(index)
            .map(|b| &mut **b)
    }

    fn len(&self) -> usize {
        self.len
    }

    fn set_page(&mut self, _index: usize, _page: usize) {
        panic!("RealMemory does not have page table");
    }

    fn del_page(&mut self, _index: usize) {
        panic!("RealMemory does not have page table");
    }

    fn as_cursor(&mut self) -> MemoryCursor {
        MemoryCursor::new(self)
    }
}

impl MemoryIndex for RealMemory {
    fn check_index(&self, index: usize) -> bool {
        index >= self.len
    }

    fn index(&self, index: usize) -> &u8 {
        &self.blocks[index / BLOCK_SIZE][index % BLOCK_SIZE]
    }

    fn index_mut(&mut self, index: usize) -> &mut u8 {
        &mut self.blocks[index / BLOCK_SIZE][index % BLOCK_SIZE]
    }
}
