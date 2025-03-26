use lexer::{
    stream::peekable::cursor::Cursor,
    token::{
        number::{Float, Integer},
        string::StringToken,
        TokenBox,
    },
};
use error::{NoneError, Result};

use super::expr::Expr;

#[derive(Debug)]
pub enum Literal {
    Integer(String),
    Float(String),
    String(String),
    // Bool(bool),
}

impl Literal {
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> Result<Box<Expr>> {
        if let Ok(integer) = cursor.eat_type::<Integer>() {
            return Ok(Box::new(Expr::Literal(Literal::Integer(integer.number()))));
        }
        if let Ok(float) = cursor.eat_type::<Float>() {
            return Ok(Box::new(Expr::Literal(Literal::Float(float.number()))));
        }
        if let Ok(string) = cursor.eat_type::<StringToken>() {
            return Ok(Box::new(Expr::Literal(Literal::String(string.string()))));
        }
        Err(NoneError.into())
    }
}
