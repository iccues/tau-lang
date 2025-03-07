pub mod peeker;
pub mod cursor;

use cursor::Cursor;

use crate::error::IResult;

use super::Stream;

pub trait Peekable: Stream {
    fn peek(&mut self) -> IResult<Self::Item>;
    fn peek_n(&mut self, n: usize) -> IResult<Self::Item>;
    fn cursor(&mut self) -> Cursor<Self::Item> where Self: Sized {
        Cursor::new(self)
    }
}