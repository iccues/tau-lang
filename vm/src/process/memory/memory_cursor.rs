use std::io::{Read, Write};

use super::{Memory, BLOCK_SIZE};

pub struct MemoryCursor<'a> {
    memory: &'a mut dyn Memory,
    pos: usize,
}

impl<'a> MemoryCursor<'a> {
    pub fn new(memory: &'a mut dyn Memory) -> Self {
        Self {
            memory,
            pos: 0,
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn seek(&mut self, pos: usize) -> &mut Self {
        self.pos = pos;
        self
    }
}

impl<'a> Write for MemoryCursor<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let block = self.memory
            .get_block(self.pos / BLOCK_SIZE)
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::OutOfMemory,
                "OutOfMemory"
            ))?;

        let copy_start = self.pos % BLOCK_SIZE;
        let copy_len = std::cmp::min(BLOCK_SIZE - copy_start, buf.len());
        let copy_end = copy_start + copy_len;

        block[copy_start..copy_end].copy_from_slice(&buf[..copy_len]);
        self.pos += copy_len;

        Ok(copy_len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a> Read for MemoryCursor<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let block = self.memory
            .get_block(self.pos / BLOCK_SIZE)
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::OutOfMemory,
                "OutOfMemory"
            ))?;

        let copy_start = self.pos % BLOCK_SIZE;
        let copy_len = std::cmp::min(BLOCK_SIZE - copy_start, buf.len());
        let copy_end = copy_start + copy_len;

        buf[..copy_len].copy_from_slice(&block[copy_start..copy_end]);
        self.pos += copy_len;

        Ok(copy_len)
    }
}
