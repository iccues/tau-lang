use std::collections::VecDeque;

use error::Result;

use crate::stream::{ErrorStream, Position, Stream};

use super::Peekable;

pub struct Peeker<I: Clone> {
    inner: Box<dyn Stream<Item = I>>,
    buffer: VecDeque<(Position, Position, Result<I>)>,
}

impl<I: Clone> ErrorStream for Peeker<I> {
    fn inner(&self) -> &dyn ErrorStream {
        panic!();
    }

    fn last_position(&self) -> Position {
        if self.buffer.is_empty() {
            self.inner.last_position()
        } else {
            self.buffer[0].0
        }
    }

    fn next_position(&self) -> Position {
        if self.buffer.is_empty() {
            self.inner.next_position()
        } else {
            self.buffer[0].1
        }
    }
}

impl<I: Clone> Stream for Peeker<I> {
    type Item = I;

    fn next(&mut self) -> Result<Self::Item> {
        self.next()
    }
}

impl<I: Clone> Peekable for Peeker<I> {
    fn peek(&mut self) -> Result<Self::Item> {
        self.peek()
    }

    fn peek_n(&mut self, n: usize) -> Result<Self::Item> {
        self.peek_n(n)
    }

    fn peeks<'a>(&'a mut self, n: usize) -> Result<Vec<Self::Item>> {
        for _ in self.buffer.len()..n {
            self.get_next();
        }

        self.buffer
            .iter()
            .take(n)
            .map(|x| x.2.clone())
            .collect::<Result<Vec<I>>>()
    }
}

impl<I: Clone> Peeker<I> {
    pub fn new(inner: impl Stream<Item = I> + 'static) -> Self {
        Self {
            inner: Box::new(inner),
            buffer: VecDeque::new(),
        }
    }

    fn get_next(&mut self) {
        let last_position = self.inner.last_position();
        let next_position = self.inner.next_position();
        let item = self.inner.next();
        self.buffer.push_back((last_position, next_position, item));
    }

    pub fn peek(&mut self) -> Result<I> {
        if self.buffer.is_empty() {
            self.get_next();
        }
        self.buffer[0].2.clone()
    }

    pub fn peek_n(&mut self, n: usize) -> Result<I> {
        for _ in self.buffer.len()..n + 1 {
            self.get_next();
        }
        self.buffer[n].2.clone()
    }

    pub fn next(&mut self) -> Result<I> {
        if self.buffer.is_empty() {
            self.get_next();
        }
        self.buffer.pop_front().unwrap().2
    }
}

impl Peeker<char> {
    pub fn peeks(&mut self, n: usize) -> Result<String> {
        for _ in self.buffer.len()..n {
            self.get_next();
        }
        let a: Vec<Result<char>> = self.buffer.iter().take(n).map(|x| x.2.clone()).collect();
        a.into_iter().collect()
    }
}
