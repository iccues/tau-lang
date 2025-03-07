pub mod number;
pub mod boxs;
pub mod function;

pub trait AsBytes {
    fn as_bytes(&self) -> Vec<u8>;
    fn as_usize(&self) -> usize {
        let bytes = self.as_bytes();
        let mut result = 0;
        for (i, byte) in bytes.iter().enumerate() {
            result += (*byte as usize) << (i * 8);
        }
        result
    }
}