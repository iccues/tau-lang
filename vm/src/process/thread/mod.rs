pub mod thread_stack;
pub mod thread_status;

use thread_stack::ThreadStack;

use librarys::signal::Signal;
use thread_status::ThreadStatus;

use crate::process::memory::*;
use crate::process::Process;

#[derive(Debug)]
pub struct Thread {
    pub pc: usize,
    pub stack: ThreadStack,

    pub memory: *mut dyn Memory,
    pub process: *mut Process,

    pub status: ThreadStatus,

    pub signal: Option<Signal>,
}

impl Thread {
    pub fn new(process: *mut Process) -> Self {
        let memory: *mut dyn Memory = unsafe { (*process).memory() };
        Thread {
            pc: 0,
            stack: ThreadStack::new(),
            memory,
            process,
            status: ThreadStatus::Sleeping,
            signal: None,
        }
    }

    pub fn memory(&self) -> &mut dyn Memory {
        unsafe { &mut *self.memory }
    }

    pub fn process(&self) -> &mut Process {
        unsafe { &mut *self.process }
    }
    
    pub fn signal(&mut self, signal: Signal) {
        self.signal = Some(signal);
    }
}