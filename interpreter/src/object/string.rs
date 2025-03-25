use super::Object;

#[derive(Debug)]
pub struct String_ {
    value: String,
}

impl String_ {
    pub fn new(literal: &str) -> Self {
        let value = literal[1..literal.len() - 1].to_string();
        String_ { value }
    }

    pub fn to_string_row(&self) -> String {
        self.value.clone()
    }
}

impl Object for String_ {
    fn to_string_row(&self) -> String {
        self.value.clone()
    }

    fn get_member(&self, name: &str) -> super::ObjectBox {
        match name {
            _ => panic!("Member not found"),
        }
    }
}
