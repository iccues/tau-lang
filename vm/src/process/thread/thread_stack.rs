use std::ops::Index;
use std::ops::IndexMut;

use librarys::signal::SignalError;
use librarys::signal::SignalResult;


#[derive(Debug)]
pub struct ThreadStack {
    stack: Vec<usize>,
}

impl ThreadStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    // pop
    pub fn pop(&mut self) -> SignalResult<usize> {
        self.stack.pop().ok_or(SignalError::StackEmpty.into())
    }

    pub fn pops(&mut self, len: usize) -> SignalResult<Vec<usize>> {
        if self.stack.len() < len {
            return Err(SignalError::StackEmpty.into());
        }
        let mut values = Vec::with_capacity(len);
        for _ in 0..len {
            values.push(self.stack.pop().unwrap());
        }
        values.reverse();
        Ok(values)
    }

    pub fn pop_args(&mut self) -> SignalResult<Vec<usize>> {
        let len = self.pop()?;
        let vec = self.pops(len)?;
        Ok(vec)
    }


    // push
    pub fn push(&mut self, value: usize) {
        self.stack.push(value);
    }
    
    pub fn pushs(&mut self, values: &[usize]) {
        self.stack.extend(values);
    }

    pub fn push_args(&mut self, values: &[usize]) {
        self.pushs(values);
        self.push(values.len());
    }

    pub fn extend(&mut self, offset: usize) {
        let new_len = self.stack.len().wrapping_add(offset);
        self.stack.resize(new_len, 0);
    }
}

impl Index<usize> for ThreadStack {
    type Output = usize;
    fn index(&self, index: usize) -> &Self::Output {
        let index = (self.stack.len() - 1) - index;
        &self.stack[index]
    }
}

impl IndexMut<usize> for ThreadStack {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let index = (self.stack.len() - 1) - index;
        &mut self.stack[index]
    }
}