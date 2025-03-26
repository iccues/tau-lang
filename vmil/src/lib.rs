pub mod table;
pub mod var;
pub mod path;
pub mod types;
pub mod as_bytes;
pub mod serialize;

pub mod a {
    use error::Error;
    use lexer::stream::Stream;

    use crate::{serialize::StrSerialize, table::{self, Table}};

    fn deserialize(filename: &str) -> Result<Table, Error> {
        let file = std::fs::File::open(filename)?;
        let reader = std::io::BufReader::new(file);
        let char_stream = lexer::stream::char_stream::CharStream::new(reader).peeker();
        let mut lexer = lexer::stream::token_stream::lexer::Lexer::new(char_stream).peeker();
        table::Table::deserialize(&mut lexer)
    }

    #[test]
    fn debug_() {
        let table = deserialize("../tests/hello.il");
        println!("{:?}", table);
    }
}