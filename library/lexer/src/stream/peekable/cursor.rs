use error::{NoneError, Result};

use crate::{stream::Stream, token::{ComplexBox, Token, TokenBox}};

use super::{super::Position, Peekable};

pub struct Cursor<'a, I: Clone> {
    peeker: &'a mut dyn Peekable<Item = I>,
    cursor: usize,
}

impl<I: Clone> Stream for Cursor<'_, I> {
    type Item = I;
    fn next(&mut self) -> Result<Self::Item> {
        self.next()
    }

    fn last_position(&self) -> Position {
        self.peeker.last_position()
    }
    fn next_position(&self) -> Position {
        self.peeker.next_position()
    }
}

impl<T: Clone> Peekable for Cursor<'_, T> {
    fn peek(&mut self) -> Result<Self::Item> {
        self.peek()
    }

    fn peek_n(&mut self, n: usize) -> Result<Self::Item> {
        self.peeker.peek_n(n)
    }

    fn peeks(&mut self, n: usize) -> Result<Vec<Self::Item>> {
        Ok(self.peeker.peeks(n + self.cursor)?.into_iter().skip(self.cursor).collect())
    }
}

impl<'a, I: Clone> Cursor<'a, I> {
    pub fn new(peeker: &'a mut dyn Peekable<Item = I>) -> Self {
        Self {
            peeker,
            cursor: 0,
        }
    }

    pub fn peek(&mut self) -> Result<I> {
        self.peeker.peek_n(self.cursor)
    }

    pub fn next(&mut self) -> Result<I> {
        self.cursor += 1;
        self.peeker.peek_n(self.cursor - 1)
    }

    pub fn sync(&mut self) {
        for _ in 0..self.cursor {
            let _ = self.peeker.next();
        }
        self.cursor = 0;
    }

    pub fn reset(&mut self) {
        self.cursor = 0;
    }

    pub fn last_position(&self) -> Position {
        self.peeker.last_position()
    }

    pub fn next_position(&self) -> Position {
        self.peeker.next_position()
    }
}

impl Cursor<'_, TokenBox> {
    pub fn eat_type<T: Token>(&mut self) -> Result<ComplexBox<T>> {
        if let Some(item) = self.peek()?.downcast() {
            let _ = self.next();
            Ok(item)
        } else {
            Err(NoneError.into())
        }
    }

    pub fn eat_eq(&mut self, value: &dyn Token) -> Result<()> {
        if self.peek()?.eq(value) {
            let _ = self.next();
            Ok(())
        } else {
            Err(NoneError.into())
        }
    }
}