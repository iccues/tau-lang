use enum_plus_derive::EnumPlus;

use crate::{bit_vec::{ucode, AsBits, BitVec}, bit_vec_builder, instruction::{arg::Arg, opcode::code::Code}};

#[allow(non_camel_case_types)]
#[derive(EnumPlus, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Syscall {

    // Call
    CALL_P = 0x00,
    RET_P = 0x01,

    // Interrupt
    INT = 0x10,
    INT_RET = 0x11,

    // Memory
    SET_PAGE = 0x20,
    DEL_PAGE = 0x21,

    // Process
    NEW_CHILD = 0x30,
    DEL_CHILD = 0x31,

    // Threads
    NEW_THREAD = 0x40,
    DEL_THREAD = 0x41,

    // IO
    PRINT = 0x80,
    PRINT_C = 0x81,
}

impl AsBits for Syscall {
    fn as_bits(&self) -> ucode {
        *self as u8 as ucode
    }

    fn boxed(&self) -> Box<dyn AsBits> {
        Box::new(*self)
    }
}

impl Syscall {
    pub fn handle(self, args: &[Arg]) -> BitVec {
        if args.len() > 0 {
            panic!();
        }
        bit_vec_builder!{
            #[32]
            [0..8] : Code::SYSCALL;
            [8..32] : self;
        }
    }
}