use std::io::{BufRead, Write};

mod line;

use crate::{
    error::Error,
    global::{Global, GlobalRc},
};
use line::LineParser;
use nom::Finish;

pub struct Parser {
    line_parser: LineParser,
    global: GlobalRc,
}

impl Parser {
    pub fn new() -> Self {
        let global = Global::new_rc();
        Self {
            line_parser: LineParser::new(global.clone()),
            global,
        }
    }

    pub fn parse<T: BufRead, U: Write>(&self, source: T, target: &mut U) {
        for (index, line) in source.lines().enumerate() {
            let line = line.unwrap();
            // self.handle_line(&line).unwrap();
            match self.handle_line(&line) {
                Ok(()) => {}
                Err(e) => {
                    e.handle(index, &line);
                    return;
                }
            }
        }
        self.global.byte_vec().write(target).unwrap();
    }

    fn handle_line<'a>(&'a self, line: &'a str) -> Result<(), Error> {
        let (_, line) = self.line_parser.parse(line).finish()?;
        line.handle(&self.global);
        Ok(())
    }
}

trait Handle {
    fn handle(&self, global: &GlobalRc);
}
