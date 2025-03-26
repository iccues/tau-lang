use std::collections::HashMap;
use std::fmt::Debug;
use crate::signal_table::types::Type;
use lexer::stream::peekable::cursor::Cursor;
use lexer::token::identifier::Identifier;
use lexer::token::keyword::Keyword;
use error::{ErrKind, NoneError, Result, ResultExt};
use lexer::token::operator::Operator;
use lexer::token::singleton_token::EofToken;
use lexer::token::TokenBox;
use crate::try_parse;

use super::func::Func;
use super::variable::Variable;

#[derive(Debug)]
pub enum ModuleItem {
    Module(Module),
    Type(Type),
    Variable(Variable),
    Func(Func),
}

impl ModuleItem {
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> Result<(ModuleItem, String)> {
        try_parse!(Module::parse(cursor));
        try_parse!(Type::parse(cursor));
        try_parse!(Variable::parse(cursor));
        try_parse!(Func::parse(cursor));
        Err(NoneError.into())
    }
}



#[derive(Debug)]
pub struct Module {
    pub map: HashMap<String, ModuleItem>,
}

impl Module {
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> Result<(ModuleItem, String)> {
        cursor.eat_eq(&Keyword::Mod)?;
        let name = cursor.eat_type::<Identifier>()?.name();
        cursor.eat_eq(&Operator::OpenBrace)?;
        cursor.sync();

        let module = Module::parser_inner(cursor).failure()?;
        cursor.eat_eq(&Operator::CloseBrace).failure()?;

        Ok((ModuleItem::Module(module), name))
    }

    pub fn parser_inner(cursor: &mut Cursor<TokenBox>) -> Result<Module> {
        let mut map = HashMap::new();

        while !cursor.peek()?.eq(&Operator::CloseBrace) && !cursor.peek()?.is::<EofToken>() {
            let (item, name) = ModuleItem::parse(cursor)?;
            map.insert(name, item);
        }

        Ok(Module { map })
    }
}

// impl Debug for Module {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Module {:?}", self.map)
//     }
// }