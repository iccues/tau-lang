use compiler::parser;
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
}
