use compiler::parser;
use std::env::args;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        panic!();
    }

    parser(&args[1]);
}
