use std::rc::Rc;

use super::Token;
use super::ComplexBox;

#[derive(Debug, PartialEq)]
pub struct Comment {
    pub content: Option<String>,
}

impl Comment {
    pub fn new(content: Option<String>) -> ComplexBox<dyn Token> {
        ComplexBox::Rc(Rc::new(Comment { content }))
    }
}

impl Token for Comment {}