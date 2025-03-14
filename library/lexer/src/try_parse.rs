#[macro_export]
macro_rules! try_parse {
    ($item:expr) => {
        match $item {
            Ok(factor) => {
                return Ok(factor);
            }
            Err(e @ error::ErrKind::Failure(_)) => {
                return Err(e);
            }
            _ => {}
        }
    };
}
