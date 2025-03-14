pub mod peeker;
pub mod cursor;

use cursor::Cursor;
use error::Result;
use super::Stream;

pub type Peek<T> = dyn Peekable<Item = T>;

pub trait Peekable: Stream {
    fn peek(&mut self) -> Result<Self::Item>;
    fn peek_n(&mut self, n: usize) -> Result<Self::Item>;
    fn peeks(&mut self, n: usize) -> Result<Vec<Self::Item>>;

    fn cursor(&mut self) -> Cursor<Self::Item> where Self: Sized {
        Cursor::new(self)
    }
}

