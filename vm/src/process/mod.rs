pub mod memory;
pub mod thread;
pub mod threads;

use std::ops::DerefMut;

use memory::*;
use library::signal::{SignalError, SignalResult};
use threads::Threads;

#[derive(Debug)]
pub struct Process {
    memory: Box<dyn Memory>,
    super_process: Option<*mut Process>,

    children: Vec<Option<Box<Process>>>,
    pub threads: Threads,

}

impl Process {
    pub fn new_root(memory_size: usize) -> Box<Process> {
        Process::from(Box::new(RealMemory::with_len(memory_size)), Option::None)
    }

    pub fn new(super_process: *mut Process) -> Box<Process> {
        let memory: *mut dyn Memory = unsafe { (*super_process).memory.deref_mut() };
        Process::from(
            Box::new(VisualMemroy::new(memory)),
            Option::Some(super_process),
        )
    }

    pub fn from(memory: Box<dyn Memory>, super_process: Option<*mut Process>) -> Box<Process> {
        let mut process = Box::new(Process {
            memory,
            super_process,
            children: Vec::new(),
            threads: Threads::new(),
        });
        let process_ptr: *mut Process = &mut *process;
        process.threads.build_process(process_ptr);
        process
    }

    pub fn memory(&mut self) -> &mut dyn Memory {
        self.memory.deref_mut()
    }

    pub fn new_child(&mut self) -> SignalResult<usize> {
        let super_process: *mut Process = self;
        self.children.push(Some(Process::new(super_process)));
        Ok(self.children.len() - 1)
    }

    pub fn del_child(&mut self, index: usize) -> SignalResult<()> {
        if self.children.len() <= index || self.children[index].is_none() {
            return Err(SignalError::ChildUnexist.into());
        }
        self.children[index] = None;
        Ok(())
    }

    pub fn get_child(&mut self, index: usize) -> SignalResult<&mut Box<Process>> {
        if self.children.len() <= index || self.children[index].is_none() {
            return Err(SignalError::ChildUnexist.into());
        }
        Ok(self.children[index].as_mut().unwrap())
    }
}
