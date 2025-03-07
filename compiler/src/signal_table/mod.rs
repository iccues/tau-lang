#![allow(dead_code)]

// mod callable;
mod types;
mod variable;
mod traits;
mod module;
mod func;
mod path;

use module::Module;

use crate::{error::IResult, stream::peekable::cursor::Cursor, token::{singleton_token::EofToken, TokenBox}};

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
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> IResult<SignalTable> {
        let inner = Module::parser_inner(cursor)?;
        cursor.eat_eq(&EofToken)?;
        Ok(SignalTable { inner, })
    }
}
