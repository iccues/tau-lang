pub mod error;
pub use self::error::SignalError;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Signal {
    Error(SignalError),
    Others(usize),
}

pub type SignalResult<T> = Result<T, Signal>;

impl From<usize> for Signal {
    fn from(value: usize) -> Self {
        if let Ok(value) = value.try_into() {
            return Signal::Error(value);
        }
        Signal::Others(value)
    }
}

impl From<Signal> for usize {
    fn from(value: Signal) -> Self {
        match value {
            Signal::Error(value) => value as usize,
            Signal::Others(value) => value,
        }
    }
}

impl From<SignalError> for Signal {
    fn from(value: SignalError) -> Self {
        Signal::Error(value)
    }
}

// impl<T> From<Result<T, Error>> for Result<T, signal> {
//     fn from(value: Result<T, Error>) -> Self {
//         match value {
//             Ok(value) => Ok(value),
//             Err(value) => Err(signal::Error(value)),
//         }
//     }
// }