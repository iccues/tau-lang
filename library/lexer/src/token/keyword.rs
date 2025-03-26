use crate::stream::peekable::cursor::Cursor;

use super::identifier::Identifier;
use super::Token;
use super::ComplexBox;
use super::TokenBox;

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Let,
    Mod,
    Def,
    Type,
    Var,
    Func,
    If,
    Else,
    While,
    Self_,
}

const LET_KEYWORD: Keyword = Keyword::Let;
const MOD_KEYWORD: Keyword = Keyword::Mod;
const DEF_KEYWORD: Keyword = Keyword::Def;
const TYPE_KEYWORD: Keyword = Keyword::Type;
const VAR_KEYWORD: Keyword = Keyword::Var;
const FUNC_KEYWORD: Keyword = Keyword::Func;
const IF_KEYWORD: Keyword = Keyword::If;
const ELSE_KEYWORD: Keyword = Keyword::Else;
const WHILE_KEYWORD: Keyword = Keyword::While;
const SELF_KEYWORD: Keyword = Keyword::Self_;

impl Keyword {
    pub fn parse(cursor: &mut Cursor<TokenBox>) -> Option<TokenBox> {
        if let Ok(identifier) = cursor.eat_type::<Identifier>() {
            match &identifier.name()[..] {
                "let" => Some(ComplexBox::Ref(&LET_KEYWORD)),
                "mod" => Some(ComplexBox::Ref(&MOD_KEYWORD)),
                "def" => Some(ComplexBox::Ref(&DEF_KEYWORD)),
                "type" => Some(ComplexBox::Ref(&TYPE_KEYWORD)),
                "var" => Some(ComplexBox::Ref(&VAR_KEYWORD)),
                "func" => Some(ComplexBox::Ref(&FUNC_KEYWORD)),
                "if" => Some(ComplexBox::Ref(&IF_KEYWORD)),
                "else" => Some(ComplexBox::Ref(&ELSE_KEYWORD)),
                "while" => Some(ComplexBox::Ref(&WHILE_KEYWORD)),
                "self" => Some(ComplexBox::Ref(&SELF_KEYWORD)),
                _ => {
                    cursor.reset();
                    None
                },
            }
        }
        else {
            None
        }
    }
}

impl Token for Keyword {}