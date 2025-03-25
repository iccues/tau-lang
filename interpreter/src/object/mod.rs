use std::{any::Any, fmt::Debug};

use downcast_rs::{impl_downcast, Downcast};
use object_fn::ObjectFnBox;

pub mod string;
pub mod number;
pub mod object_fn;
pub mod tuple;

pub type ObjectBox = Box<dyn Object>;

pub trait Object: Downcast + Debug {
    fn to_string_row(&self) -> String;
    fn get_member(&self, _name: &str) -> ObjectBox {
        panic!("Member not found");
    }
    fn call(&self, _input: ObjectBox) -> ObjectBox {
        panic!("Not callable");
    }
}

impl_downcast!(Object);


#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::number::Integer;
    use crate::object::tuple::Tuple;

    #[test]
    fn test_integer() {
        let integer = Integer::new("42");
        assert_eq!(integer.to_string_row(), "42");
    }

    #[test]
    fn test_tuple() {
        let tuple = Tuple::new(vec![
            Box::new(Integer::new("42")),
            Box::new(Integer::new("24")),
        ]);
        assert_eq!(tuple.to_string_row(), "(42, 24)");
    }

    #[test]
    fn test_integer_add() {
        let integer = Integer::new("42");
        let result = integer.get_member("add").call(Box::new(Tuple::new(vec![
            Box::new(Integer::new("42")),
            Box::new(Integer::new("24")),
        ])));
        assert_eq!(result.to_string_row(), "66");
    }
}