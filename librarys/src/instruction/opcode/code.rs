use asm_macro::code_rules;
use enum_plus_derive::EnumPlus;

#[derive(EnumPlus, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Code {
    ADD = 0x00,
    SUB = 0x01,
    AND = 0x02,
    OR = 0x03,
    NOT = 0x04,
    INCR = 0x05,
    XOR = 0x06,
    SHL = 0x07,
    SHR = 0x08,

    IF = 0x10,
    JUMP = 0x11,
    END = 0x12,

    // Load and Store
    LOAD_8 = 0x20,
    STORE_8 = 0x21,
    LOAD_16 = 0x22,
    STORE_16 = 0x23,
    LOAD_32 = 0x24,
    STORE_32 = 0x25,
    LOAD_64 = 0x26,
    STORE_64 = 0x27,
    LOAD_I = 0x30,

    // Stack
    EXTEND = 0x40,
    EXTEND_I = 0x41,
    PUSH = 0x42,
    POP = 0x43,
    CALL_F = 0x44,
    RET_F = 0x45,

    SYSCALL = 0x80,
}

use crate::{bit_vec::{ucode, AsBits, BitVec}, instruction::arg::Arg};

impl AsBits for Code {
    fn as_bits(&self) -> ucode {
        *self as u8 as ucode
    }

    fn boxed(&self) -> Box<dyn AsBits> {
        Box::new(*self)
    }
}

impl Code {
    pub fn handle(self, args: &[Arg]) -> BitVec {
        code_rules! {
            match (self, args) {
                (Code::ADD, [Reg, Reg, Reg]) => #32{
                    [0..8] : code;
                    [8..16] : args[0];
                    [16..24] : args[1];
                    [24..32] : args[2];
                },
                (Code::SUB, [Reg, Reg, Reg]) => #32{
                    [0..8] : code;
                    [8..16] : args[0];
                    [16..24] : args[1];
                    [24..32] : args[2];
                },
                (Code::AND, [Reg, Reg, Reg]) => #32{
                    [0..8] : code;
                    [8..16] : args[0];
                    [16..24] : args[1];
                    [24..32] : args[2];
                },
                (Code::OR, [Reg, Reg, Reg]) => #32{
                    [0..8] : code;
                    [8..16] : args[0];
                    [16..24] : args[1];
                    [24..32] : args[2];
                },
                (Code::XOR, [Reg, Reg, Reg]) => #32{
                    [0..8] : code;
                    [8..16] : args[0];
                    [16..24] : args[1];
                    [24..32] : args[2];
                },
                (Code::NOT, [Reg, Reg]) => #32{
                    [0..8] : code;
                    [8..16] : args[0];
                    [16..24] : args[1];
                },
                (Code::INCR, [Reg]) => #32{
                    [0..8] : code;
                    [8..16] : args[0];
                },
                (Code::SHL, [Reg, Reg, Reg]) => #32{
                    [0..8] : code;
                    [8..16] : args[0];
                    [16..24] : args[1];
                    [24..32] : args[2];
                },
                (Code::SHR, [Reg, Reg, Reg]) => #32{
                    [0..8] : code;
                    [8..16] : args[0];
                    [16..24] : args[1];
                    [24..32] : args[2];
                },


                (Code::IF, [Reg]) => #32{
                    [0..8] : code;
                    [8..16] : args[0];
                },
                (Code::JUMP, [Num]) => #32{
                    [0..8] : code;
                    [8..32] : args[0];
                },
                (Code::END, []) => #32{
                    [0..8] : code;
                },


                (Code::LOAD_8, [Reg, Num]) => #32{
                    [0..8] : code;
                    [8..16] : args[0];
                    [16..24] : args[1];
                },
                (Code::LOAD_16, [Reg, Num]) => #32{
                    [0..8] : code;
                    [8..16] : args[0];
                    [16..24] : args[1];
                },
                (Code::LOAD_32, [Reg, Num]) => #32{
                    [0..8] : code;
                    [8..16] : args[0];
                    [16..24] : args[1];
                },
                (Code::LOAD_64, [Reg, Num]) => #32{
                    [0..8] : code;
                    [8..16] : args[0];
                    [16..24] : args[1];
                },
                (Code::STORE_8, [Reg, Num]) => #32{
                    [0..8] : code;
                    [8..16] : args[0];
                    [16..24] : args[1];
                },
                (Code::STORE_16, [Reg, Num]) => #32{
                    [0..8] : code;
                    [8..16] : args[0];
                    [16..24] : args[1];
                },
                (Code::STORE_32, [Reg, Num]) => #32{
                    [0..8] : code;
                    [8..16] : args[0];
                    [16..24] : args[1];
                },
                (Code::STORE_64, [Reg, Num]) => #32{
                    [0..8] : code;
                    [8..16] : args[0];
                    [16..24] : args[1];
                },
                (Code::LOAD_I, [Reg, Num]) => #32{
                    [0..8] : code;
                    [8..16] : args[0];
                    [16..24] : args[1];
                },

                (Code::EXTEND, [Reg]) => #32{
                    [0..8] : code;
                    [8..16] : args[0];
                },
                (Code::EXTEND_I, [Num]) => #32{
                    [0..8] : code;
                    [8..32] : args[0];
                },
                (Code::PUSH, [Reg]) => #32{
                    [0..8] : code;
                    // TODO
                },
                (Code::POP, [Reg]) => #32{
                    [0..8] : code;
                    // TODO
                },


                (Code::CALL_F, [Num]) => #32{
                    [0..8] : code;
                    [8..32] : args[0];
                },
                (Code::RET_F, []) => #32{
                    [0..8] : code;
                },



                (Code::SYSCALL, [Num]) => #32{
                    [0..8] : code;
                    [8..32] : args[0];
                },
            }
        }
    }
}