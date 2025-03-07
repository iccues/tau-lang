use std::io::Write;

use librarys::bit_vec::BitVec;

pub struct ByteVec {
    buf: Vec<BitVec>,
    len: usize,
}

impl ByteVec {
    pub fn new() -> ByteVec {
        ByteVec {
            buf: vec![],
            len: 0,
        }
    }

    pub fn push(&mut self, value: BitVec) {
        self.len += value.len() / 8;
        self.buf.push(value);
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn write<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        for bit_vec in &self.buf {
            writer.write_all(&bit_vec.as_bytes())?;
        }
        Ok(())
    }
}
