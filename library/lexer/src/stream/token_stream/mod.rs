use error::{NoneError, Result};

use crate::token::{ComplexBox, Token, TokenBox};

use super::peekable::Peek;

pub mod lexer;
pub mod token_processor;

impl Peek<TokenBox> {
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