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

impl<E: std::error::Error> From<E> for ErrKind<GError> {
    fn from(value: E) -> Self {
        value.into()
    }
}