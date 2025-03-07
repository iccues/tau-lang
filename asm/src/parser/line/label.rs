use nom::{
    character::complete::{char, space0},
    IResult,
};

use crate::global::symbol_table::SymbolTableEntry;
use crate::{global::GlobalRc, parser::Handle};

use crate::error::Error;

use super::parse_ident;

#[derive(Debug)]
pub struct Label {
    entry: SymbolTableEntry,
}

impl Label {
    pub fn set(&self, value: usize) {
        self.entry.set(value);
    }
}

impl Handle for Label {
    fn handle(&self, global: &GlobalRc) {
        self.set(global.byte_vec().len());
    }
}

pub struct LabelParser {
    global: GlobalRc,
}

impl<'a> LabelParser {
    pub fn new(global: GlobalRc) -> Self {
        Self { global }
    }

    pub fn parse(&self, input: &'a str) -> IResult<&'a str, Label, Error> {
        let (input, _) = space0(input)?;
        let (input, name) = parse_ident(input)?;
        let (input, _) = char(':')(input)?;
        let (input, _) = space0(input)?;

        let label = Label {
            entry: self.global.symbol_table().entry(name),
        };
        Ok((input, label))
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_label() {
//         let parser = LabelParser::new();

//         let input = "label1_aaa_234:";
//         let expected = Ok(("", Label {
//             name: "label1_aaa_234".to_string(),
//             value: None,
//         }));
//         assert_eq!(parser.parse(input), expected);
//     }
// }
