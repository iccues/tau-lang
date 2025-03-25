use super::{Object, ObjectBox};

pub type ObjectFn = fn(ObjectBox) -> ObjectBox;
pub type ObjectFnBox = Box<ObjectFn>;

impl Object for ObjectFn {
    fn to_string_row(&self) -> String {
        "function".to_string()
    }

    fn call(&self, input: ObjectBox) -> ObjectBox {
        self(input)
    }
}

pub fn print_func(input: ObjectBox) -> ObjectBox {
    println!("{}", input.to_string_row());
    input
}
