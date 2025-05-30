use std::ops::Deref;

use error::Result;
use lexer::token::operator::Operator;

use crate::{as_bytes::AsBytes, path::Path, serialize::StrSerialize, types::Types};

#[derive(Debug)]
pub struct Var {
    var_type: Box<Types>,
    pub value: Value,
}

#[derive(Debug)]
pub enum Value {
    //
    // Bytes(Box<dyn AsBytes>),
    // *
    HeapDeref(Box<Value>),
    // %
    StackDeref(Box<Value>),
    // &
    HeapBox(Box<Value>),
    StackBox(Box<Value>),
    // use
    Use(Box<Var>),
    UsePath(Path),
}

impl Value {
    pub fn get(&self) {}
    pub fn set(&mut self) {}
}

impl StrSerialize for Value {
    fn serialize(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        // match self {
        //     Value::Bytes(bytes) => bytes.serialize(writer),
        //     Value::HeapDeref(value) => {
        //         writer.write_all(b"*")?;
        //         value.serialize(writer)
        //     }
        //     Value::StackDeref(value) => {
        //         writer.write_all(b"%")?;
        //         value.serialize(writer)
        //     }
        //     Value::HeapBox(value) => {
        //         writer.write_all(b"&")?;
        //         value.serialize(writer)
        //     }
        //     Value::StackBox(value) => {
        //         writer.write_all(b"$")?;
        //         value.serialize(writer)
        //     }
        //     Value::Use(var) => {
        //         writer.write_all(b"@")?;
        //         var.serialize(writer)
        //     }
        //     Value::UsePath(path) => path.serialize(writer),
        // }
        unimplemented!();
    }

    // fn deserialize(reader: &mut dyn std::io::BufRead) -> Self {
    //     // let mut buf = [0; 1];
    //     // reader.read_exact(&mut buf).unwrap();
    //     // match buf[0] {
    //     //     b'*' => Value::HeapDeref(Box::new(Value::deserialize(reader))),
    //     //     b'%' => Value::StackDeref(Box::new(Value::deserialize(reader))),
    //     //     b'&' => Value::HeapBox(Box::new(Value::deserialize(reader))),
    //     //     b'$' => Value::StackBox(Box::new(Value::deserialize(reader))),
    //     //     b'@' => Value::Use(Box::new(Var::deserialize(reader))),
    //     //     _ => Value::Bytes(Box::new(Vec::<u8>::deserialize(reader))),
    //     // }
    //     unimplemented!();
    // }

    // fn deserialize(reader: &mut lexer::stream::peekable::Peek<char>) -> Result<Self> {
    //     match reader.peek()? {
    //         '*' => {
    //             reader.next()?;
    //             Ok(Value::HeapDeref(Box::new(Value::deserialize(reader)?)))
    //         }
    //         '%' => {
    //             reader.next()?;
    //             Ok(Value::StackDeref(Box::new(Value::deserialize(reader)?)))
    //         }
    //         _ => unimplemented!(),
    //     }
    // }

    fn deserialize(reader: &mut lexer::stream::peekable::Peek<lexer::token::TokenBox>) -> Result<Self> {
        if let Ok(op) = reader.eat_type::<Operator>() {
            match op.deref() {
                &Operator::Star => {
                    return Ok(Value::HeapDeref(Box::new(Value::deserialize(reader)?)));
                }
                _ => unimplemented!(),
            }
        };
        unimplemented!();
    }

}