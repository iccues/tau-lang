use std::ptr::null_mut;

use librarys::signal::{SignalError, SignalResult};

use crate::process::Process;
use super::thread::Thread;

#[derive(Debug)]
pub struct Threads {
    threads: Vec<Option<Thread>>,
    process: *mut Process,
}

impl Threads {
    pub fn new() -> Threads {
        Threads {
            threads: vec![],
            process: null_mut(),
        }
    }

    pub fn build_process(&mut self, process: *mut Process) {
        self.process = process;
    }

    pub fn new_thread(&mut self) -> SignalResult<usize> {
        self.threads.push(Some(Thread::new(self.process)));
        Ok(self.threads.len() - 1)
    }

    pub fn del_thread(&mut self, thread: usize) -> SignalResult<()> {
        if self.threads.len() <= thread || self.threads[thread].is_none() {
            return Err(SignalError::ThreadNotFound.into());
        }
        self.threads[thread] = None;
        Ok(())
    }

    pub fn get_thread(&mut self, thread: usize) -> SignalResult<&mut Thread> {
        match self.threads.get_mut(thread) {
            Some(Some(thread)) => Ok(thread),
            _ => Err(SignalError::ThreadNotFound.into()),
        }
    }
}
