use block::Block;

use super::AsBytes;

pub mod instruction;
pub mod block;

pub struct Function {
    pub block: Block,
}

impl AsBytes for Function {
    fn as_bytes(&self) -> Vec<u8> {
        unimplemented!()
    }
}