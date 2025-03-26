use lexer::{stream::peekable::cursor::Cursor, token::{operator::Operator, TokenBox}};
use error::Result;

use super::expr::Expr;

pub enum Stmt {
    Expr(Box<Expr>),
    Def,
    
}

impl Stmt {
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> Result<Stmt> {
        let expr = Expr::parse(cursor)?;
        cursor.eat_eq(&Operator::Semi)?;
        Ok(Stmt::Expr(expr))
    }
}