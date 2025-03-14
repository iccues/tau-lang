use error::Result;
use peekable::peeker::Peeker;

pub mod char_stream;
pub mod token_stream;
pub mod peekable;

pub trait ErrorStream {
    fn inner(&self) -> &dyn ErrorStream;
    fn last_position(&self) -> Position {
        self.inner().last_position()
    }
    fn next_position(&self) -> Position {
        self.inner().next_position()
    }
}

pub trait Stream: ErrorStream {
    type Item: Clone;

    fn next(&mut self) -> Result<Self::Item>;
    fn nexts(&mut self, n: usize) -> Result<Vec<Self::Item>> {
        let mut items = Vec::with_capacity(n);
        for _ in 0..n {
            items.push(self.next()?);
        }
        Ok(items)
    }

    fn peeker(self) -> Peeker<Self::Item> where Self: Sized + 'static {
        Peeker::new(self)
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    line: usize,
    column: usize,
}

impl Position {
    pub fn new() -> Self {
        Self { line: 0, column: 0 }
    }

    pub fn move_right(&mut self) {
        self.column += 1;
    }

    pub fn move_down(&mut self) {
        self.line += 1;
        self.column = 0;
    }
}
