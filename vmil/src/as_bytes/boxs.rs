use crate::types::Types;

use super::AsBytes;

pub struct HeapBox {
    inner: Box<dyn AsBytes>,
}

impl AsBytes for HeapBox {
    fn as_bytes(&self) -> Vec<u8> {
        // self.inner.as_bytes()
        unimplemented!()
    }
}

pub struct StackBox {
    type_: Types,
}