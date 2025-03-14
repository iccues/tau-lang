pub mod arg;
pub mod opcode;
pub mod error;

use arg::Arg;
use opcode::Opcode;

use crate::bit_vec::BitVec;

pub struct Instruction {
    pub opcode: Opcode,
    pub args: Vec<Arg>,
}

impl Instruction {
    pub fn as_bit_vec(&self) -> BitVec {
        match self.opcode {
            Opcode::Code(code) => code.handle(&self.args),
            Opcode::Syscall(syscall) => syscall.handle(&self.args),
        }
    }
}
