use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1},
    character::complete::{char, digit1, space0},
    multi::separated_list0,
    IResult,
};

use crate::global::symbol_table::SymbolTableEntry;
use crate::{error::ResultExt, global::GlobalRc, parser::line::parse_ident};
use library::bit_vec::{ucode, AsBits};

use crate::error::Error;

use library::instruction::arg::Arg;

pub struct ArgParser {
    global: GlobalRc,
}

#[derive(Debug, Clone)]
pub enum Num {
    Num(ucode),
    Symbol(SymbolTableEntry),
}

impl AsBits for Num {
    fn as_bits(&self) -> ucode {
        match self {
            Num::Num(i) => *i as ucode,
            Num::Symbol(entry) => entry.get().unwrap() as ucode, // TODO
        }
    }

    fn boxed(&self) -> Box<dyn AsBits> {
        Box::new(self.clone())
    }
}

impl ArgParser {
    pub fn new(global: GlobalRc) -> Self {
        Self { global }
    }

    fn parse_num_fn(prefix: String, radix: ucode) -> impl Fn(&str) -> IResult<&str, u32, Error> {
        move |input_: &str| {
            let (input, _) = tag(prefix.as_str())(input_)?;
            let (input, num) = take_till1(|c: char| c.is_whitespace() || c == ',')(input)?;
            let (input, _) = space0(input)?;
            Ok((input, ucode::from_str_radix(num, radix).as_err(input_)?))
        }
    }

    pub fn parse_num(input: &str) -> IResult<&str, Arg, Error> {
        let (_, _) = digit1(input)?;
        let (input, num) = alt((
            Self::parse_num_fn("0x".to_string(), 16),
            Self::parse_num_fn("0b".to_string(), 2),
            Self::parse_num_fn("0o".to_string(), 8),
            Self::parse_num_fn("".to_string(), 10),
        ))(input)?;
        Ok((input, Arg::Num(Num::Num(num).boxed())))
    }

    pub fn parse_reg(input: &str) -> IResult<&str, Arg, Error> {
        let (input, _) = char('%')(input)?;
        let (input, reg) = parse_ident(input)?;
        let reg = reg.try_into().unwrap();
        Ok((input, Arg::Reg(reg)))
    }

    pub fn parse_symbol<'a>(&self, input: &'a str) -> IResult<&'a str, Arg, Error> {
        let (input, _) = char('$')(input)?;
        let (input, name) = parse_ident(input)?;
        let (input, _) = space0(input)?;
        Ok((
            input,
            Arg::Num(Num::Symbol(self.global.symbol_table().entry(name)).boxed()),
        ))
    }

    pub fn parse<'a>(&self, input: &'a str) -> IResult<&'a str, Arg, Error> {
        let (input, _) = space0(input)?;
        let (input, arg) = alt((Self::parse_num, Self::parse_reg, |input_| {
            self.parse_symbol(input_)
        }))(input)?;
        let (input, _) = space0(input)?;

        Ok((input, arg))
    }

    pub fn parsers<'a>(&self, input: &'a str) -> IResult<&'a str, Vec<Arg>, Error> {
        let (input, _) = space0(input)?;
        let (input, args) = separated_list0(char(','), |input_| self.parse(input_))(input)?;
        let (input, _) = space0(input)?;
        Ok((input, args))
    }
}
