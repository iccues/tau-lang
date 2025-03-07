use nom::{error::ParseError, Err};

pub trait ResultExt {
    type Value;
    fn as_err(self, position: &str) -> Result<Self::Value, nom::Err<Error>>;
}

impl ResultExt for Result<u32, std::num::ParseIntError> {
    type Value = u32;
    fn as_err(self, position: &str) -> Result<Self::Value, nom::Err<Error>> {
        match self {
            Ok(value) => Ok(value),
            Err(_) => Err(Err::Failure(Error::new(position))),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    // row: Option<usize>,
    // column: Option<usize>,
    position: *const [u8],
}

impl Error {
    pub fn new(position: &str) -> Self {
        Self {
            // row: None,
            // column: None,
            position: position.as_bytes(),
        }
    }

    pub fn handle(self, row: usize, line: &str) {
        // self.row = Some(row);
        // self.column =
        println!(
            "Error from row: {}, column: {:?}",
            row,
            self.where_str(line)
        );
        println!("code: {}", &line[self.where_str(line)]);
    }

    fn where_str(&self, line: &str) -> std::ops::Range<usize> {
        let line: *const [u8] = line.as_bytes();
        // if self.position < line && self.position + self.position.len() > line + line.len() {}
        // let start = (self.position as *const u8).wrapping_sub(line as *const u8 as usize);
        let line_start = line as *const u8 as usize;
        let error_start = self.position as *const u8 as usize;
        // let line_end = line_start + line.len(); // TODO
        let error_end = error_start + self.position.len();
        // if error_start < line_start || error_end > line_end {
        //     return 0..0;
        // }
        let start = error_start - line_start;
        let end = error_end - line_start;
        start..end
    }
}

// impl Display for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         Ok(())
//     }
// }

impl ParseError<&str> for Error {
    fn from_error_kind(input: &str, _kind: nom::error::ErrorKind) -> Self {
        Self::new(input)
    }

    fn append(_input: &str, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

// impl From<nom::Err<nom::error::Error<&str>>> for Error {
//     fn from(value: nom::Err<nom::error::Error<&str>>) -> Self {
//         let position = match value {
//             nom::Err::Failure(e) | nom::Err::Error(e) => e,
//             nom::Err::Incomplete(_) => panic!(),
//         }.input.as_bytes();

//         Self {
//             // row: None,
//             // column: None,
//             position,
//         }
//     }
// }

// impl From<Error> for nom::Err<Error> {
//     fn from(value: Error) -> Self {
//         Err::Error(value)
//     }
// }
