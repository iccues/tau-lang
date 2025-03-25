use compiler::signal_table::func::literal::Literal;

pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Array(Vec<Value>),
    Tuple(Vec<Value>),
    Function,
}

impl Value {
    pub fn new(literal: Literal) -> Self {
        match literal {
            Literal::Integer(i) => {
                unimplemented!()
            }
            Literal::Float(f) => {
                // Value::Float(f.into)
                unimplemented!()
            }
            Literal::String(s) => {
                Value::String(s)
            }
        }
    }
}