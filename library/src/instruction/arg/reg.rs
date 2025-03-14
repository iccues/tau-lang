use enum_plus_derive::EnumPlusStr;

use crate::bit_vec::AsBits;

use crate::bit_vec::ucode;

#[derive(EnumPlusStr, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum Reg {
    r0,
    r1,
    r2,
    r3,
    r4,
    r5,
    r6,
    r7,
    r8,
    r9,
    r10,
    r11,
    r12,
    r13,
    r14,
    r15,
}

impl AsBits for Reg {
    fn as_bits(&self) -> ucode {
        match self {
            Reg::r0 => 0x00,
            Reg::r1 => 0x01,
            Reg::r2 => 0x02,
            Reg::r3 => 0x03,
            Reg::r4 => 0x04,
            Reg::r5 => 0x05,
            Reg::r6 => 0x06,
            Reg::r7 => 0x07,
            Reg::r8 => 0x08,
            Reg::r9 => 0x09,
            Reg::r10 => 0x0A,
            Reg::r11 => 0x0B,
            Reg::r12 => 0x0C,
            Reg::r13 => 0x0D,
            Reg::r14 => 0x0E,
            Reg::r15 => 0x0F,
        }
    }

    fn boxed(&self) -> Box<dyn AsBits> {
        Box::new(self.clone())
    }
}