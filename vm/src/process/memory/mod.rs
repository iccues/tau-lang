use std::fmt::Debug;

mod real_memory;
mod visual_memory;
mod memory_cursor;

use memory_cursor::MemoryCursor;
use library::signal::{SignalError, SignalResult};
pub(crate) use real_memory::RealMemory;
pub(crate) use visual_memory::VisualMemroy;

const BLOCK_SIZE: usize = 4;
type Block = [u8; BLOCK_SIZE];

#[allow(dead_code)]
pub trait Memory: MemoryIndex + Debug {
    fn get_block(&mut self, index: usize) -> Option<&mut Block>;
    fn get_block_ptr(&mut self, index: usize) -> Option<*mut Block> {
        self.get_block(index).map(|b| b as *mut Block)
    }
    fn len(&self) -> usize; // TODO

    fn set_page(&mut self, index: usize, page: usize);

    fn del_page(&mut self, index: usize);

    fn as_cursor(&mut self) -> MemoryCursor;
}

pub trait MemoryIndex {

    fn check_index(&self, index: usize) -> bool;
    fn index(&self, index: usize) -> &u8;
    fn index_mut(&mut self, index: usize) -> &mut u8;

    // fn get_u8(&self, index: usize) -> Result<u8, ()> {
    //     if self.check_index(index) {
    //         return Err(());
    //     }
    //     Ok(*self.index(index))
    // }

    fn get_u8(&self, index: usize) -> SignalResult<usize> {
        if self.check_index(index) {
            return Err(SignalError::MemoryOutOfBounds.into());
        }
        Ok(*self.index(index) as usize)
    }

    fn set_u8(&mut self, index: usize, value: u8) -> SignalResult<()> {
        if self.check_index(index) {
            return Err(SignalError::MemoryOutOfBounds.into());
        }
        *self.index_mut(index) = value;
        Ok(())
    }
}