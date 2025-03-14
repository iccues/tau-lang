use library::signal::{SignalError, SignalResult};

use super::Thread;

#[derive(Debug)]
pub enum ThreadStatus {
    Sleeping,
    Running,
    Interrupted(Vec<*mut Thread>),
}

impl ThreadStatus {
    pub fn is_interrupted(&self) -> SignalResult<&Vec<*mut Thread>> {
        match self {
            ThreadStatus::Interrupted(threads) => Ok(threads),
            _ => Err(SignalError::ThreadNotInterrupted.into()),
        }
    }
}