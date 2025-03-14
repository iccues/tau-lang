pub mod code;
pub mod syscall;

use code::Code;
use syscall::Syscall;

#[derive(Debug, PartialEq)]
pub enum Opcode {
    Code(Code),
    Syscall(Syscall),
}

impl TryFrom<&str> for Opcode {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(code) = Code::try_from(value) {
            return Ok(Self::Code(code));
        }
        if let Ok(syscall) = Syscall::try_from(value) {
            return Ok(Self::Syscall(syscall));
        }
        
        Err(())
    }
}
