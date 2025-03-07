use std::{fmt::Display, rc::Rc};

// pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    IO(Rc<std::io::Error>),
    UnkonwnToken,

    None,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(e) => write!(f, "IO Error: {}", e),
            Self::UnkonwnToken => write!(f, "Unknown Token"),
            Self::None => write!(f, "None"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IO(Rc::new(e))
    }
}


#[derive(Debug, Clone)]
pub enum ErrKind<E> {
    Error(E),
    Failure(E),
}

impl From<std::io::Error> for ErrKind<Error> {
    fn from(e: std::io::Error) -> Self {
        Self::Failure(Error::IO(Rc::new(e)))
    }
}


pub type IResult<T, E = Error> = std::result::Result<T, ErrKind<E>>;

pub trait IResultExt {
    fn failure(self) -> Self;
}

impl<T, E> IResultExt for IResult<T, E> {
    fn failure(self) -> Self {
        match self {
            Err(ErrKind::Error(e)) => Err(ErrKind::Failure(e)),
            r => r,
        }
    }
}
