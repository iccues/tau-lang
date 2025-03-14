mod bit_vec;
mod bit_vec_builder;

pub use bit_vec::BitVec;

#[allow(non_camel_case_types)]
pub type ucode = u32;

pub trait AsBits {
    fn as_bits(&self) -> ucode;

    fn boxed(&self) -> Box<dyn AsBits>;
}

impl AsBits for ucode {
    fn as_bits(&self) -> ucode {
        *self
    }

    fn boxed(&self) -> Box<dyn AsBits> {
        Box::new(*self)
    }
}
