use error::{Result, ResultExt};
use lexer::stream::peekable::cursor::Cursor;
use lexer::token::operator::Operator;
use lexer::token::{identifier::Identifier, TokenBox};

#[derive(Debug)]
pub struct Path {
    pub names: Vec<String>,
}

impl Path {
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> Result<Path> {
        let mut names = vec![];

        names.push(cursor.eat_type::<Identifier>()?.name());
        cursor.sync();

        while cursor.eat_eq(&Operator::DoubleColon).is_ok() {
            names.push(cursor.eat_type::<Identifier>().failure()?.name());
            cursor.sync();
            
        }

        Ok(Path {
            names,
        })
    }

}