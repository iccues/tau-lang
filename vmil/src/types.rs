#[allow(non_camel_case_types)]
pub enum Types {
    Function(Vec<Types>, Box<Types>),
    
    // Variable
    u8,
    u16,
    u32,
    u64,
    usize,
    i8,
    i16,
    i32,
    i64,
    isize,
    char8,
    char32,
    f32,
    f64,
    bool,

    Pointer(Box<Types>),
    Array(Box<Types>, usize),
    Tuple(Vec<Types>),
}