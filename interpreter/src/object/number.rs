use crate::object::tuple::Tuple;

use super::{object_fn::ObjectFn, Object, ObjectBox};

#[derive(Debug)]
pub struct Integer {
    value: i64,
}

impl Integer {
    pub fn new(literal: &str) -> Self {
        let value = literal.parse::<i64>().unwrap();
        Integer { value }
    }

    pub fn add(input: ObjectBox) -> ObjectBox {
        match input.downcast::<Tuple>().unwrap().deconst() {
            [a, b] => {
                let a = (&**a).downcast_ref::<Integer>().unwrap();
                let b = (&**b).downcast_ref::<Integer>().unwrap();
                let result = a.value + b.value;
                Box::new(Integer { value: result })
            }
            _ => panic!("Invalid input"),
        }
    }
}

impl Object for Integer {
    fn to_string_row(&self) -> String {
        self.value.to_string()
    }

    fn get_member(&self, name: &str) -> super::ObjectBox {
        match name {
            "add" => Box::new(Integer::add as ObjectFn),
            _ => panic!("Member not found"),
        }
    }
}
