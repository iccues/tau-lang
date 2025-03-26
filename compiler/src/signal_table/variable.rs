use error::Result;
use lexer::token::identifier::Identifier;
use lexer::token::TokenBox;
use lexer::stream::peekable::cursor::Cursor;
use lexer::token::keyword::Keyword;
use lexer::token::operator::Operator;

use super::module::ModuleItem;
use super::path::Path;

#[derive(Debug)]
pub struct Variable {
    var_type: Path,
}

impl Variable {
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> Result<(ModuleItem, String)> {
        cursor.eat_eq(&Keyword::Var)?;
        let name = cursor.eat_type::<Identifier>()?.name();
        cursor.eat_eq(&Operator::Colon)?;
        let var_type = Path::parse(cursor)?;
        cursor.eat_eq(&Operator::Semi)?;

        cursor.sync();

        Ok((ModuleItem::Variable(Variable { var_type }), name))
    }
}