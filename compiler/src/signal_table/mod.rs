#![allow(dead_code)]

mod types;
mod variable;
mod module;
mod func;
mod path;

use module::Module;

use lexer::{stream::peekable::cursor::Cursor, token::{singleton_token::EofToken, TokenBox}};
use error::Result;

#[macro_export]
macro_rules! try_parse {
    ($item:expr) => {
        match $item {
            Ok(factor) => {
                return Ok(factor);
            }
            Err(e @ ErrKind::Failure(_)) => {
                return Err(e);
            }
            _ => {}
        }
    };
}

#[derive(Debug)]
pub struct SignalTable {
    pub inner: Module,
}

impl SignalTable {
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> Result<SignalTable> {
        let inner = Module::parser_inner(cursor)?;
        cursor.eat_eq(&EofToken)?;
        Ok(SignalTable { inner, })
    }
}
