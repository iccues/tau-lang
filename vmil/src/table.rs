use std::collections::HashMap;

use crate::{serialize::StrSerialize, var::Value};

pub struct Table {
    pub value: Option<Value>,
    table: HashMap<String, Table>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            value: None,
            table: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: Table) {
        self.table.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&Table> {
        self.table.get(key)
    }
}

impl StrSerialize for Table {
    fn serialize(&self, writer: &mut dyn std::io::Write) -> std::io::Result<()> {
        unimplemented!();
    }

    // fn deserialize(reader: &mut stream::peekable::Peek<char>) -> error::Result<Self> {
    //     reader.eat_whitespace()?;
    //     reader.eat_eq(&'{')?;
    //     reader.eat_whitespace()?;

    //     let mut value = None;
    //     if reader.eat_str("self").is_ok() {
    //         reader.eat_whitespace()?;
    //         reader.eat_eq(&':')?;
    //         value = Some(Value::deserialize(reader)?);
    //         reader.eat_eq(&',')?;
    //     }
        
    //     let mut table = HashMap::new();
    //     // Todo: Deserialize table

    //     Ok(Table {
    //         value,
    //         table,
    //     })
    // }

    fn deserialize(reader: &mut lexer::stream::peekable::Peek<lexer::token::TokenBox>) -> error::Result<Self> {
        unimplemented!()
    }
}
