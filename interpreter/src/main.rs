use compiler::{parser, signal_table::{module::ModuleItem, SignalTable}};
use std::env::args;

mod value;
mod object;
mod expr_eval;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        panic!();
    }

    let signal_table = parser(&args[1]);
    run(signal_table);
}

fn run(signal_table: SignalTable) {
    let main = signal_table.inner.map.get("main").unwrap();
    let ModuleItem::Func(func) = main else {
        panic!();
    };
}