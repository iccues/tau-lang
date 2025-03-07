use crate::error::IResult;
use crate::stream::peekable::cursor::Cursor;
use crate::token::{operator::Operator, TokenBox};

use super::expr::Expr;

#[derive(Debug)]
pub struct Block {
    statments: Vec<Box<Expr>>,
    end_value: Option<Box<Expr>>,
}

impl Block {
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> IResult<Box<Expr>> {
        cursor.eat_eq(&Operator::OpenBrace)?;

        let mut statments = Vec::new();
        let mut end_value = None;

        while cursor.eat_eq(&Operator::CloseBrace).is_err() {
            let statment = Expr::parse(cursor)?;
            if cursor.eat_eq(&Operator::Semi).is_err() {
                end_value = Some(statment);
                cursor.eat_eq(&Operator::CloseBrace)?;
                break;
            }
            statments.push(statment);
            cursor.sync();
        }

        cursor.sync();
        Ok(Box::new(Expr::Block(Block {
            statments,
            end_value,
        })))
    }
}