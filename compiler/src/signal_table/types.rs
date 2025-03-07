// use std::collections::HashMap;
// use crate::signal_table::callable::Callable;
// 
// pub trait Type {
//     fn get_size(&self) -> usize;
//     fn get_fn(&self, name: &str) -> &Callable;
// }
// 
// pub struct Type0 {
//     size: usize,
//     implement: HashMap<String, Callable>,
// }
// 
// impl Type for Type0 {
//     fn get_size(&self) -> usize {
//         self.size
//     }
// 
//     fn get_fn(&self, name: &str) -> &Callable {
//         &self.implement[name]
//     }
// }

use crate::{error::IResult, stream::peekable::cursor::Cursor, token::{identifier::Identifier, keyword::Keyword, operator::Operator, TokenBox}};

use super::module::ModuleItem;

#[derive(Debug)]
pub struct Type {}

impl Type {
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> IResult<(ModuleItem, String)> {
        cursor.eat_eq(&Keyword::Type)?;
        let name = cursor.eat_type::<Identifier>()?.name();
        cursor.eat_eq(&Operator::Semi)?;

        cursor.sync();

        Ok((ModuleItem::Type(Type {}), name))
    }
}