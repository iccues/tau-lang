use crate::{error::IResult, stream::peekable::cursor::Cursor, token::{operator::Operator, TokenBox}};

use super::expr::Expr;

pub enum Stmt {
    Expr(Box<Expr>),
    Def,
    
}

impl Stmt {
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> IResult<Stmt> {
        let expr = Expr::parse(cursor)?;
        cursor.eat_eq(&Operator::Semi)?;
        Ok(Stmt::Expr(expr))
    }
}