pub(crate) mod arg;
pub(crate) mod opcode;

use arg::ArgParser;
use nom::{character::complete::space0, IResult};
use opcode::OpcodeParser;

use crate::{global::GlobalRc, parser::Handle};

use crate::error::Error;

pub use library::instruction::Instruction;

impl Handle for Instruction {
    fn handle(&self, global: &GlobalRc) {
        global.byte_vec().push(self.as_bit_vec());
    }
}

pub struct InstructionParser {
    args_parser: ArgParser,
    opcode_parser: OpcodeParser,
    #[allow(dead_code)]
    global: GlobalRc,
}

impl<'a> InstructionParser {
    pub fn new(global: GlobalRc) -> Self {
        Self {
            args_parser: ArgParser::new(global.clone()),
            opcode_parser: OpcodeParser::new(global.clone()),
            global,
        }
    }

    pub fn parse(&self, input: &'a str) -> IResult<&'a str, Instruction, Error> {
        let (input, _) = space0(input)?;
        let (input, opcode) = self.opcode_parser.parse(input)?;
        let (input, args) = self.args_parser.parsers(input)?;
        Ok((input, Instruction { opcode, args }))
    }
}
