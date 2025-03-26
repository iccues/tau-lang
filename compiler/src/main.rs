use std::fs::File;
use std::env::args;
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
mod signal_table;

// mod error;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        panic!();
    }

    parser(&args[1]);
    // print_token(&args[1]);

}

fn parser(input: &str) {
    let input = File::open(input).unwrap();
    let char_stream = CharStream::new(BufReader::new(input)).peeker();
    let lexer = Lexer::new(char_stream).peeker();
    let mut token_processor = TokenProcessor::new(lexer).peeker();

    let mut cursor = token_processor.cursor();

    let signal_table = SignalTable::parse(&mut cursor).unwrap();
    println!("{:?}", signal_table);
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
        // println!("{}", std::env::current_dir().unwrap().display());
        parser("../tests/hello.plang");
    }
}