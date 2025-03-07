use crate::error::IResult;
use crate::stream::peekable::cursor::Cursor;
use crate::token::{operator::Operator, keyword::Keyword, TokenBox};

use super::expr::Expr;

#[derive(Debug)]
pub struct IfExpr {
    condition: Box<Expr>,
    then_block: Box<Expr>,
    else_block: Option<Box<Expr>>,
}

impl IfExpr {
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> IResult<Box<Expr>> {
        cursor.eat_eq(&Keyword::If)?;
        cursor.eat_eq(&Operator::OpenParen)?;
        let condition = Expr::parse(cursor)?;
        cursor.eat_eq(&Operator::CloseParen)?;
        let then_block = Expr::parse(cursor)?;
        let else_block = if cursor.eat_eq(&Keyword::Else).is_ok() {
            Some(Expr::parse(cursor)?)
        } else {
            None
        };
        Ok(Box::new(Expr::If(
            IfExpr {
                condition, then_block, else_block
            }
        )))
    }
}