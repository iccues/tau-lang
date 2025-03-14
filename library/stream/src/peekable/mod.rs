pub mod peeker;
pub mod cursor;

use cursor::Cursor;
use error::{ErrKind, NoneError, Result};
use super::Stream;

pub type Peek<T> = dyn Peekable<Item = T>;

pub trait Peekable: Stream {
    fn peek(&mut self) -> Result<Self::Item>;
    fn peek_n(&mut self, n: usize) -> Result<Self::Item>;
    fn peeks(&mut self, n: usize) -> Result<Vec<Self::Item>>;

    fn cursor(&mut self) -> Cursor<Self::Item> where Self: Sized {
        Cursor::new(self)
    }

    fn eat_eq(&mut self, item: &dyn EqTo<Self::Item>) -> Result<()> {
        if item.eq_to(&self.peek()?) {
            self.next()?;
            Ok(())
        } else {
            Err(ErrKind::Error(NoneError.into()))
        }
    }
}

pub trait EqTo<T> {
    fn eq_to(&self, other: &T) -> bool;
}

impl<T: Eq> EqTo<T> for T {
    fn eq_to(&self, other: &T) -> bool {
        self == other
    }
}
