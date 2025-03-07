use crate::error::IResult;
use crate::stream::peekable::cursor::Cursor;
use crate::token::{operator::Operator, keyword::Keyword, TokenBox};

use super::expr::Expr;

#[derive(Debug)]
pub struct WhileExpr {
    condition: Box<Expr>,
    then_block: Box<Expr>,
}

impl WhileExpr {
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> IResult<Box<Expr>> {
        cursor.eat_eq(&Keyword::While)?;
        cursor.eat_eq(&Operator::OpenParen)?;
        let condition = Expr::parse(cursor)?;
        cursor.eat_eq(&Operator::CloseParen)?;
        let then_block = Expr::parse(cursor)?;
        Ok(Box::new(Expr::While(WhileExpr { condition, then_block })))
    }
}