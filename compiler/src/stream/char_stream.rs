use std::io::{BufRead, Lines};

use crate::error::IResult;
use super::{Position, ErrorStream, Stream};

// pub mod char_peeker;


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

    pub fn next(&mut self) -> IResult<char> {
        self.position.move_right();
        if self.position.column >= self.line.len() {
            if let Some(line) = self.reader.next() {
                self.line = line?.chars().collect();
                self.line.push('\n');
                self.position.move_down();
            }
            else {
                return Ok(EOF_CHAR);
            }
        }
        Ok(self.line[self.position.column])
    }
}

impl Stream for CharStream {
    type Item = char;

    fn next(&mut self) -> IResult<Self::Item> {
        self.next()
    }
}

impl ErrorStream for CharStream {
    fn inner(&self) -> &dyn ErrorStream {
        panic!();
    }

    fn last_position(&self) -> Position {
        self.position
    }

    fn next_position(&self) -> Position {
        self.position
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