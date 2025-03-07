use crate::process::thread::Thread;

pub struct IlVpu {
    stack: Vec<*mut Thread>,
}

impl IlVpu {
    pub fn new() -> Self {
        Self {
            stack: vec![],
        }
    }

    pub fn run(&mut self) {
        unimplemented!()
    }
}