use std::env::args;
use std::fs::File;
use std::io::BufReader;

mod error;
mod global;
mod parser;

fn main() {
    println!("Hello from asm!");

    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        panic!();
    }

    parser(&args[1], &args[2]);

    println!("Success!");
}

fn parser(input_file: &str, output_file: &str) {
    let input_file = File::open(input_file).unwrap();
    let mut output_file = File::create(output_file).unwrap();

    let parser = parser::Parser::new();
    parser.parse(BufReader::new(input_file), &mut output_file);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        parser("tests/hello.asm", "tests/hello1.bin");
    }
}
