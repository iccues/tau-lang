use nom::IResult;
use librarys::instruction::opcode::Opcode;

use crate::{global::GlobalRc, parser::line::parse_ident};

use crate::error::Error;

pub struct OpcodeParser {
    #[allow(dead_code)]
    global: GlobalRc,
}

impl OpcodeParser {
    pub fn new(global: GlobalRc) -> Self {
        Self { global }
    }

    pub fn parse<'a>(&self, input: &'a str) -> IResult<&'a str, Opcode, Error> {
        let (input, code) = parse_ident(input)?;
        Ok((input, Opcode::try_from(code).unwrap()))
    }
}
