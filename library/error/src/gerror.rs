use std::{fmt::Display, ops::Deref, rc::Rc};


pub type GResult<T> = Result<T, GError>;

#[derive(Debug, Clone)]
pub struct GError {
    inner: Rc<dyn std::error::Error + Send + Sync>,
}

impl GError {
    pub fn new<T: std::error::Error + Send + Sync + 'static>(value: T) -> Self {
        GError {
            inner: Rc::new(value),
        }
    }
}

impl Display for GError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "GError: {}", self.inner)
    }
}

impl<T: std::error::Error + Send + Sync + 'static> From<T> for GError {
    fn from(value: T) -> Self {
        GError::new(value)
    }
}

impl Deref for GError {
    type Target = dyn std::error::Error + Send + Sync;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}
