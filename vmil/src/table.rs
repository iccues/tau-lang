use std::collections::HashMap;

use lexer::token::{identifier::Identifier, keyword::Keyword, operator::Operator};

use crate::{serialize::StrSerialize, var::Value};

#[derive(Debug)]
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

    fn deserialize(reader: &mut lexer::stream::peekable::Peek<lexer::token::TokenBox>) -> error::Result<Self> {
        reader.eat_eq(&Operator::OpenBrace)?;

        let mut value = None;
        if reader.eat_eq(&Keyword::Self_).is_ok() {
            reader.eat_eq(&Operator::Colon)?;
            value = Some(Value::deserialize(reader)?);
            reader.eat_eq(&Operator::Comma)?;
        }

        let mut table = HashMap::new();
        while reader.eat_eq(&Operator::CloseBrace).is_err() {
            let key = reader.eat_type::<Identifier>()?.name();
            reader.eat_eq(&Operator::Colon)?;
            let value = Table::deserialize(reader)?;
            reader.eat_eq(&Operator::Comma)?;

            table.insert(key, value);
        }

        Ok(Table {
            value,
            table,
        })
    }
}
