pub mod reg;

use reg::Reg;

use crate::bit_vec::{ucode, AsBits};

// #[derive(Debug)]
pub enum Arg {
    Num(Box<dyn AsBits>),
    Reg(Reg),
}

impl Clone for Arg {
    fn clone(&self) -> Self {
        match self {
            Arg::Num(i) => Arg::Num(i.boxed()),
            Arg::Reg(i) => Arg::Reg(i.clone()),
        }
    }
}

impl AsBits for Arg {
    fn as_bits(&self) -> ucode {
        match self {
            Arg::Num(i) => i.as_bits(),
            Arg::Reg(i) => i.as_bits(),
        }
    }

    fn boxed(&self) -> Box<dyn AsBits> {
        Box::new(self.clone())
    }
}
