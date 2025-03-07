use std::collections::HashMap;
use std::fmt::Debug;
use crate::signal_table::types::Type;
use crate::stream::peekable::cursor::Cursor;
use crate::token::identifier::Identifier;
use crate::token::keyword::Keyword;
use crate::error::{ErrKind, IResult, IResultExt};
use crate::token::operator::Operator;
use crate::token::singleton_token::EofToken;
use crate::token::TokenBox;
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
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> IResult<(ModuleItem, String)> {
        try_parse!(Module::parse(cursor));
        try_parse!(Type::parse(cursor));
        try_parse!(Variable::parse(cursor));
        try_parse!(Func::parse(cursor));
        Err(ErrKind::Error(crate::error::Error::None))
    }
}



#[derive(Debug)]
pub struct Module {
    pub map: HashMap<String, ModuleItem>,
}

impl Module {
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> IResult<(ModuleItem, String)> {
        cursor.eat_eq(&Keyword::Mod)?;
        let name = cursor.eat_type::<Identifier>()?.name();
        cursor.eat_eq(&Operator::OpenBrace)?;
        cursor.sync();

        let module = Module::parser_inner(cursor).failure()?;
        cursor.eat_eq(&Operator::CloseBrace).failure()?;

        Ok((ModuleItem::Module(module), name))
    }

    pub fn parser_inner(cursor: &mut Cursor<TokenBox>) -> IResult<Module> {
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