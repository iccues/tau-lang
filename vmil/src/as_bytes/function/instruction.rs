use crate::var::Value;

pub struct Instruction {
    pub opcode: Opcode,
    pub args: Vec<Value>,
    pub ret: Value,
}


impl Instruction {
    // pub fn new(opcode: Opcode, args: Vec<Value>, ret: Value) -> Self {
    //     Self {
    //         opcode,
    //         args,
    //         ret,
    //     }
    // }
}

pub enum Opcode {
    Mov,
    Add,
    Sub,
    // Mul,
    // Div,
    // Mod,
    // And,
    // Or,
    // Xor,
    // Not,
    // Shl,
    // Shr,
    // Cmp,
    // Jmp,
    // Jz,
    // Jnz,
    // Jl,
    // Jle,
    // Jg,
    // Jge,
    // Call,
    // Ret,
    // Mov,
    // Load,
    // Store,
    // Push,
    // Pop,
    // Nop,
}