use compiler::parser;
use std::env::args;
#[macro_use]
mod signal_table;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        panic!();
    }

    parser(&args[1]);
}
