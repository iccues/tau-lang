use super::{Object, ObjectBox};

#[derive(Debug)]
pub struct Tuple {
    value: Vec<ObjectBox>,
}

impl Tuple {
    pub fn new(value: Vec<ObjectBox>) -> Self {
        Tuple { value }
    }
    
    pub fn deconst(&self) -> &[ObjectBox] {
        &self.value
    }
}

impl Object for Tuple {
    fn to_string_row(&self) -> String {
        let mut result = String::from("(");
        for (i, value) in self.value.iter().enumerate() {
            result.push_str(&value.to_string_row());
            if i != self.value.len() - 1 {
                result.push_str(", ");
            }
        }
        result.push_str(")");
        result
    }

    fn get_member(&self, name: &str) -> super::ObjectBox {
        match name {
            _ => panic!("Member not found"),
        }
    }
}
