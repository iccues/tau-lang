use expr::Expr;

use crate::error::IResult;
use crate::stream::peekable::cursor::Cursor;
use crate::token::operator::Operator;
use crate::token::{identifier::Identifier, keyword::Keyword, TokenBox};

use super::module::ModuleItem;


pub mod expr;
pub mod block;
pub mod if_expr;
pub mod while_expr;
pub mod literal;
pub mod stmt;

#[derive(Debug)]
pub struct Func {
    factor: Box<Expr>,
}

impl Func {
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> IResult<(ModuleItem, String)> {
        cursor.eat_eq(&Keyword::Func)?;
        let name = cursor.eat_type::<Identifier>()?.name();
        cursor.eat_eq(&Operator::OpenParen)?;
        cursor.eat_eq(&Operator::CloseParen)?;
        let factor = Expr::parse_factor(cursor)?;
        Ok((ModuleItem::Func(Func { factor }), name))
    }
}