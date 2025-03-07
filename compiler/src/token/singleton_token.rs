use super::Token;
use super::ComplexBox;
use super::TokenBox;

#[derive(Debug, PartialEq)]
pub struct EofToken;

const EOF_TOKEN: EofToken = EofToken;

impl EofToken {
    pub fn new() -> TokenBox {
        ComplexBox::Ref(&EOF_TOKEN)
    }
}

impl Token for EofToken {}
