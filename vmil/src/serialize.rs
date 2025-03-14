use std::io::{BufRead, Write};

use error::Result;
use lexer::{stream::peekable::Peek, token::TokenBox};

pub trait StrSerialize: Sized {
    fn serialize(&self, writer: &mut dyn Write) -> std::io::Result<()>;
    fn deserialize(reader: &mut Peek<TokenBox>) -> Result<Self>;
}