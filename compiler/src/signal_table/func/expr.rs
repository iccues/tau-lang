use crate::{
    signal_table::func::{block::Block, if_expr::IfExpr, literal::Literal},
    try_parse,
};

use error::{ErrKind, NoneError, Result};
use lexer::{
    stream::peekable::cursor::Cursor,
    token::{identifier::Identifier, operator::Operator, ComplexBox, TokenBox},
};
use super::while_expr::WhileExpr;

#[derive(Debug)]
pub enum Expr {
    BinaryExpr {
        left: Box<Expr>,
        operator: ComplexBox<Operator>,
        right: Box<Expr>,
    },

    /// Factors
    Block(Block),
    If(IfExpr),
    While(WhileExpr),
    Literal(Literal),

    Identifier(String),
    UnaryExpr {
        operator: ComplexBox<Operator>,
        expr: Box<Expr>,
    },
}

impl Expr {
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> Result<Box<Expr>> {
        let mut factors = Vec::new();
        let mut operators: Vec<ComplexBox<Operator>> = Vec::new();

        factors.push(Self::parse_factor(cursor)?);

        while cursor
            .peek()?
            .downcast::<Operator>()
            .is_some_and(|o| o.priority() >= 0)
        {

            let operator = cursor.eat_type::<Operator>()?;

            while !operators.is_empty()
                && operator.priority() <= operators.last().unwrap().priority()
            {
                let right = factors.pop().unwrap();
                let left = factors.pop().unwrap();
                let operator = operators.pop().unwrap();
                factors.push(Box::new(Expr::BinaryExpr {
                    left,
                    operator,
                    right,
                }));
            }

            operators.push(operator);
            factors.push(Self::parse_factor(cursor)?);
        }

        while !operators.is_empty() {
            let right = factors.pop().unwrap();
            let left = factors.pop().unwrap();
            let operator = operators.pop().unwrap();
            factors.push(Box::new(Expr::BinaryExpr {
                left,
                operator,
                right,
            }));
        }

        Ok(factors.pop().unwrap())
    }

    pub fn parse_factor(cursor: &mut Cursor<TokenBox>) -> Result<Box<Expr>> {
        try_parse!(Block::parse(cursor));
        try_parse!(IfExpr::parse(cursor));
        try_parse!(WhileExpr::parse(cursor));
        try_parse!(Literal::parse(cursor));
        try_parse!(Self::parse_id(cursor));
        try_parse!(Self::parse_unary(cursor));
        Err(NoneError.into())
    }

    fn parse_id(cursor: &mut Cursor<TokenBox>) -> Result<Box<Expr>> {
        let id = cursor.eat_type::<Identifier>()?;
        Ok(Box::new(Expr::Identifier(id.name())))
    }

    fn parse_unary(cursor: &mut Cursor<TokenBox>) -> Result<Box<Expr>> {
        let operator = cursor.eat_type::<Operator>()?;
        let expr = Self::parse(cursor)?;
        Ok(Box::new(Expr::UnaryExpr { operator, expr }))
    }
}
