#![allow(dead_code)]

use std::env::args;
use std::fs::File;
use std::io::copy;

mod process;
mod vpu;

use process::Process;

fn main() {
    println!("Hello from VM!");

    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        panic!();
    }

    run(&args[1]);
}

fn run(path: &str) {
    let mut file = File::open(path).unwrap();

    let mut root_process = Process::new_root(1024);
    root_process.new_child().unwrap();
    root_process.threads.new_thread().unwrap();
    copy(&mut file, &mut root_process.memory().as_cursor()).unwrap();

    let mut vpu = vpu::Vpu::new();
    vpu.runs(root_process.threads.get_thread(0).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let path = "D:/Project/Rust/puzzle/tests/hello.bin";
        run(path);
    }
}
