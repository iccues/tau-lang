use std::fs::File;
use std::io::BufReader;

use signal_table::SignalTable;
use lexer::stream::char_stream::CharStream;
use lexer::stream::peekable::Peekable;
use lexer::stream::Stream;
use lexer::token::singleton_token::EofToken;
use lexer::stream::token_stream::lexer::Lexer;
use lexer::stream::token_stream::token_processor::TokenProcessor;

// mod token;
// mod stream;
#[macro_use]
pub mod signal_table;
// mod error;


pub fn parser(input: &str) -> SignalTable {
    let input = File::open(input).unwrap();
    let char_stream = CharStream::new(BufReader::new(input)).peeker();
    let lexer = Lexer::new(char_stream).peeker();
    let mut token_processor = TokenProcessor::new(lexer).peeker();

    let mut cursor = token_processor.cursor();

    SignalTable::parse(&mut cursor).unwrap()
}

#[allow(dead_code)]
fn print_token(input: &str) {
    let input = File::open(input).unwrap();
    let char_stream = CharStream::new(BufReader::new(input)).peeker();
    let lexer = Lexer::new(char_stream).peeker();
    let mut token_processor = TokenProcessor::new(lexer).peeker();

    loop {
        let token = token_processor.next().unwrap();
        if token.is::<EofToken>() {
            break;
        }
        println!("{:?}", token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        println!("{:?}", parser("../tests/hello.plang"));
    }

    #[test]
    fn test_print_token() {
        print_token("../tests/hello.plang");
    }
}