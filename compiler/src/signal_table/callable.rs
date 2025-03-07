use std::rc::Rc;
use crate::signal_table::types::Type;
use crate::token::TokenBox;

pub struct Callable {
    arguments: Vec<Rc<Type>>,
    return_type: Rc<Type>,
    inner: Vec<TokenBox>,
}