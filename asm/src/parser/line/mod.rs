pub(crate) mod instruction;
pub(crate) mod label;

use nom::{
    branch::alt,
    bytes::complete::take_while,
    character::complete::{char, satisfy, space0},
    combinator::{eof, recognize},
    sequence::pair,
    IResult,
};

use crate::error::Error;

use instruction::{Instruction, InstructionParser};

use label::{Label, LabelParser};

use crate::global::GlobalRc;

use super::Handle;

pub fn parse_ident(input: &str) -> IResult<&str, &str, Error> {
    let first_char = alt((satisfy(|c| c.is_alphabetic()), char('_')));
    let rest_chars = take_while(|c: char| c.is_alphanumeric() || c == '_');
    recognize(pair(first_char, rest_chars))(input)
}

pub enum Line {
    Instruction(Instruction),
    Label(Label),
    Blank,
}

impl Handle for Line {
    fn handle(&self, global: &GlobalRc) {
        match self {
            Line::Instruction(i) => i.handle(global),
            Line::Label(label) => label.handle(global),
            Line::Blank => {}
        }
    }
}

pub struct LineParser {
    label_parser: LabelParser,
    instruction_parser: InstructionParser,
    #[allow(dead_code)]
    global: GlobalRc,
}

impl LineParser {
    pub fn new(global: GlobalRc) -> Self {
        Self {
            label_parser: LabelParser::new(global.clone()),
            instruction_parser: InstructionParser::new(global.clone()),
            global,
        }
    }

    fn parse_label(&self) -> impl Fn(&str) -> IResult<&str, Line, Error> + '_ {
        move |input_| {
            let (input, label) = self.label_parser.parse(input_)?;
            Ok((input, Line::Label(label)))
        }
    }

    fn parse_instruction(&self) -> impl Fn(&str) -> IResult<&str, Line, Error> + '_ {
        move |input_| {
            let (input_, instruction) = self.instruction_parser.parse(input_)?;
            Ok((input_, Line::Instruction(instruction)))
        }
    }

    fn parse_blank(&self) -> impl Fn(&str) -> IResult<&str, Line, Error> + '_ {
        move |input_| {
            let (input_, _) = space0(input_)?;
            let (_, _) = eof(input_)?;
            Ok((input_, Line::Blank))
        }
    }

    pub fn parse<'a>(&self, input: &'a str) -> IResult<&'a str, Line, Error> {
        let (input, line) = alt((
            self.parse_blank(),
            self.parse_label(),
            self.parse_instruction(),
        ))(input)?;
        let (input, _) = eof(input)?;
        Ok((input, line))
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_parse_line() {
//         let _ = dbg!(Line::parse("ADD 0x1, 0x2"));
//         let _ = dbg!(Line::parse("label:"));
//         // let _ = dbg!(Line::parse(" "));
//         // let _ = dbg!(Line::parse(""));
//         // let _ = dbg!(Line::parse("MOV AX, BX, CX,"));
//         // let _ = dbg!(Line::parse("MOV AX, BX BX,"));
//     }

//     #[test]
//     fn test_parse_ident() {
//         assert_eq!(parse_ident("abc123_"), Ok(("", "abc123_")));
//         assert_eq!(parse_ident("abc_123"), Ok(("", "abc_123")));
//         assert_eq!(parse_ident("abc_123_"), Ok(("", "abc_123_")));
//         // assert_eq!(parse_ident("123_abc"), Err());
//         assert_eq!(parse_ident("_abc123"), Ok(("", "_abc123")));
//         assert_eq!(parse_ident("abc123"), Ok(("", "abc123")));
//     }
// }
