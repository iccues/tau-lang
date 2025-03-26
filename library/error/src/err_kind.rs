use crate::gerror::GError;

#[derive(Debug, Clone)]
pub enum ErrKind<E> {
    Error(E),
    Failure(E),
}

impl<E> From<E> for ErrKind<E> {
    fn from(value: E) -> Self {
        Self::Failure(value)
    }
}

impl<E: std::error::Error + Send + Sync + 'static> From<E> for ErrKind<GError> {
    fn from(value: E) -> Self {
        Self::Error(GError::new(value))
    }
}

pub trait ResultExt {
    fn failure(self) -> Self;
}

impl<T, E> ResultExt for super::Result<T, E> {
    fn failure(self) -> Self {
        match self {
            Err(ErrKind::Error(e)) => Err(ErrKind::Failure(e)),
            other => other,
        }
    }
}
