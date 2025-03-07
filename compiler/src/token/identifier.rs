use std::rc::Rc;

use super::Token;
use super::ComplexBox;


#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
}

impl Token for Identifier {}

impl Identifier {
    pub fn new(name: String) -> ComplexBox<dyn Token> {
        ComplexBox::Rc(Rc::new(Identifier { name }))
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}