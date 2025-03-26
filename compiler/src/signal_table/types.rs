use lexer::{stream::peekable::cursor::Cursor, token::{identifier::Identifier, keyword::Keyword, operator::Operator, TokenBox}};
use error::Result;

use super::module::ModuleItem;

#[derive(Debug)]
pub struct Type {}

impl Type {
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> Result<(ModuleItem, String)> {
        cursor.eat_eq(&Keyword::Type)?;
        let name = cursor.eat_type::<Identifier>()?.name();
        cursor.eat_eq(&Operator::Semi)?;

        cursor.sync();

        Ok((ModuleItem::Type(Type {}), name))
    }
}
