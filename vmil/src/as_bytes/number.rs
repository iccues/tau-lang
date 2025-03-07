use super::AsBytes;

pub struct Number {
    value: i64,
}

impl AsBytes for Number {
    fn as_bytes(&self) -> Vec<u8> {
        self.value.to_le_bytes().to_vec()
    }
}