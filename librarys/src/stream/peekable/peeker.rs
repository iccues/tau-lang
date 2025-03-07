use std::collections::VecDeque;

use crate::error::IResult;
use crate::stream::{Position, ErrorStream, Stream};

use super::cursor::Cursor;
use super::Peekable;

pub struct Peeker<T: Clone> {
    inner: Box<dyn Stream<Item = T>>,
    buffer: VecDeque<(Position, Position, IResult<T>)>,
}

impl<T: Clone> ErrorStream for Peeker<T> {
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

impl<T: Clone> Stream for Peeker<T> {
    type Item = T;
    
    fn next(&mut self) -> IResult<Self::Item> {
        self.next()
    }
}

impl<T:Clone> Peekable for Peeker<T> {
    fn peek(&mut self) -> IResult<Self::Item> {
        self.peek()
    }
    fn peek_n(&mut self, n: usize) -> IResult<Self::Item> {
        self.peek_n(n)
    }
}

impl<T: Clone> Peeker<T> {
    pub fn new(inner: impl Stream<Item = T> + 'static) -> Self {
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

    pub fn peek(&mut self) -> IResult<T> {
        if self.buffer.is_empty() {
            self.get_next();
        }
        self.buffer[0].2.clone()
    }

    pub fn peek_n(&mut self, n: usize) -> IResult<T> {
        for _ in self.buffer.len()..n + 1 {
            self.get_next();
        }
        self.buffer[n].2.clone()
    }

    pub fn next(&mut self) -> IResult<T> {
        if self.buffer.is_empty() {
            self.get_next();
        }
        self.buffer.pop_front().unwrap().2
    }

}

impl Peeker<char> {
    pub fn peeks(&mut self, n: usize) -> IResult<String> {
        for _ in self.buffer.len()..n {
            self.get_next();
        }
        let a: Vec<IResult<char>> = self
            .buffer
            .iter()
            .take(n)
            .map(|x| x.2.clone())
            .collect();
        a.into_iter().collect()
    }
}
