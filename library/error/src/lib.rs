mod gerror;
mod err_kind;
mod none_error;


pub use gerror::{GError, GResult};
pub use err_kind::ErrKind;
pub use none_error::NoneError;

pub type Result<T, E = GError> = std::result::Result<T, err_kind::ErrKind<E>>;
pub type Error<E = GError> = ErrKind<E>;