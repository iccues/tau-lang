use std::io::{BufRead, Lines};
use error::NoneError;
use error::Result;
use super::peekable::Peek;

use super::{Position, Stream};


pub const EOF_CHAR: char = '\0';

pub struct CharStream {
    reader: Lines<Box<dyn BufRead>>,
    line: Vec<char>,
    pub position: Position,
}

impl CharStream {
    pub fn new(reader: impl BufRead + 'static) -> Self {
        Self {
            reader: (Box::new(reader) as Box<dyn BufRead>).lines(),
            line: vec![],
            position: Position::new(),
        }
    }

    pub fn next(&mut self) -> Result<char> {
        self.position.move_right();
        if self.position.column >= self.line.len() {
            match self.reader.next() {
                Some(Ok(line)) => {
                    self.line = line.chars().collect();
                    self.line.push('\n');
                    self.position.move_down();
                }
                Some(Err(err)) => return Err(err.into()),
                None => return Ok(EOF_CHAR),
            }
        }
        Ok(self.line[self.position.column])
    }
}

impl Stream for CharStream {
    type Item = char;

    fn next(&mut self) -> Result<Self::Item> {
        self.next()
    }

    fn last_position(&self) -> Position {
        self.position
    }
    fn next_position(&self) -> Position {
        self.position
    }
}


impl Peek<char> {
    pub fn peek_str(&mut self, n: usize) -> Result<String> {
        Ok(self.peeks(n)?.into_iter().collect())
    }

    pub fn eat_str(&mut self, s: &str) -> Result<()> {
        if self.peek_str(s.len())? == s {
            self.nexts(s.len())?;
            Ok(())
        } else {
            Err(NoneError.into())
        }
    }

    pub fn eat_whitespace(&mut self) -> Result<()> {
        while self.peek()?.is_whitespace() {
            self.next()?;
        }
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_char_stream() {
//         let reader = std::io::Position::new("Hello world!\nH");
//         let mut char_stream = CharStream::new(Box::new(reader));

//         dbg!(char_stream.position());
//         dbg!(char_stream.peek());
//         dbg!(char_stream.position());
//         dbg!(char_stream.peek_n(5));
//         dbg!(char_stream.position());
//         dbg!(char_stream.next());
//         dbg!(char_stream.position());
//         dbg!(char_stream.next());
//         dbg!(char_stream.position());
//         dbg!(char_stream.next());
//         dbg!(char_stream.position());
//         dbg!(char_stream.next());
//         dbg!(char_stream.position());
//         dbg!(char_stream.peek());
//         dbg!(char_stream.position());
//         dbg!(char_stream.peek());
//         dbg!(char_stream.position());
//         dbg!(char_stream.next());
//         dbg!(char_stream.position());
//         dbg!(char_stream.next());
//         dbg!(char_stream.position());
//         dbg!(char_stream.peek());
//         dbg!(char_stream.position());
//         dbg!(char_stream.peek());
//         dbg!(char_stream.position());
//         dbg!(char_stream.next());
//         dbg!(char_stream.position());
//         dbg!(char_stream.next());
//         dbg!(char_stream.position());
//         dbg!(char_stream.next());
//         dbg!(char_stream.position());
//         dbg!(char_stream.next());
//         dbg!(char_stream.position());
//         dbg!(char_stream.next());
//         dbg!(char_stream.position());
//         dbg!(char_stream.next());
//         dbg!(char_stream.position());
//         dbg!(char_stream.next());
//         dbg!(char_stream.position());
//         dbg!(char_stream.next());
//         dbg!(char_stream.position());
//         dbg!(char_stream.next());
//         dbg!(char_stream.position());

//     }
// }