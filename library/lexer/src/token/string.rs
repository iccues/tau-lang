use super::Token;
use super::ComplexBox;
use std::rc::Rc;


#[derive(Debug, PartialEq)]
pub struct StringToken {
    string: String,
}

impl StringToken {
    pub fn new(string: String) -> ComplexBox<dyn Token> {
        ComplexBox::Rc(Rc::new(Self { string }))
    }

    pub fn string(&self) -> String {
        self.string.clone()
    }
}

impl Token for StringToken {}