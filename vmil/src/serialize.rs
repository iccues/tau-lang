use std::io::{BufRead, Write};

pub trait StrSerialize {
    fn serialize(&self, writer: &mut dyn Write) -> std::io::Result<()>;
    fn deserialize(reader: &mut dyn BufRead) -> Self;
}