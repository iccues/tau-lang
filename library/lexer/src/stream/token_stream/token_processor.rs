use error::Result;

use crate::stream::peekable::Peekable;
use crate::stream::{ErrorStream, peekable::peeker::Peeker, Stream};
use crate::token::operator::Operator;
use crate::token::TokenBox;
use crate::token::keyword::Keyword;

pub struct TokenProcessor {
    input: Peeker<TokenBox>,
}

impl Stream for TokenProcessor {
    type Item = TokenBox;

    fn next(&mut self) -> Result<Self::Item> {
        self.next_token()
    }
}

impl ErrorStream for TokenProcessor {
    fn inner(&self) -> &dyn ErrorStream {
        &self.input
    }
}

impl TokenProcessor {
    pub fn new(input: Peeker<TokenBox>) -> Self {
        Self { input }
    }

    pub fn next_token(&mut self) -> Result<TokenBox> {
        let cursor = &mut self.input.cursor();

        if let Some(t) = Keyword::parse(cursor) {
            cursor.sync();
            return Ok(t);
        }

        if let Some(t) = Operator::complex_parse(cursor) {
            cursor.sync();
            return Ok(t);
        }

        self.input.next()
    }
}