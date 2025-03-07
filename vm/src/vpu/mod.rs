mod syscall;
mod ret_call_0;
mod ilvpu;

use librarys::{instruction::opcode::code::Code, signal::{Signal, SignalResult}};

use crate::process::thread::{thread_status::ThreadStatus, Thread};

pub struct Vpu {
    stack: Vec<*mut Thread>,
}

impl Vpu {
    pub fn new() -> Self {
        Self {
            stack: vec![],
        }
    }

    fn stack_head(&mut self) -> &mut Thread {
        unsafe { &mut **self.stack.last_mut().unwrap() }
    }

    fn run(&mut self) -> SignalResult<()> {
        let thread = self.stack_head();
        let code: Result<Code, ()> = thread.memory().get_u8(thread.pc)?.try_into();
        let a1 = thread.memory().get_u8(thread.pc + 1)?;
        let a2 = thread.memory().get_u8(thread.pc + 2)?;
        let a3 = thread.memory().get_u8(thread.pc + 3)?;
        match code {
            Ok(Code::ADD) => {
                self.stack_head().stack[a1] = self.stack_head().stack[a2] + self.stack_head().stack[a3];
            }
            Ok(Code::SUB) => {
                self.stack_head().stack[a1] = self.stack_head().stack[a2] - self.stack_head().stack[a3];
            }
            Ok(Code::AND) => {
                self.stack_head().stack[a1] = self.stack_head().stack[a2] & self.stack_head().stack[a3];
            }
            Ok(Code::OR) => {
                self.stack_head().stack[a1] = self.stack_head().stack[a2] | self.stack_head().stack[a3];
            }
            Ok(Code::NOT) => {
                self.stack_head().stack[a1] = !self.stack_head().stack[a2];
            }
            Ok(Code::INCR) => {
                self.stack_head().stack[a1] += 1;
            }
            Ok(Code::XOR) => {
                self.stack_head().stack[a1] = self.stack_head().stack[a2] ^ self.stack_head().stack[a3];
            }
            Ok(Code::SHL) => {
                self.stack_head().stack[a1] = self.stack_head().stack[a2] << self.stack_head().stack[a3];
            }
            Ok(Code::SHR) => {
                self.stack_head().stack[a1] = self.stack_head().stack[a2] >> self.stack_head().stack[a3];
            }

            // Ok(Code::IF) => {}
            Ok(Code::JUMP) => {
                self.stack_head().pc = self.stack_head().stack[a1];
            }
            // Ok(Code::END) => {}
            Ok(Code::LOAD_8) => {
                let i = self.stack_head().stack[a2];
                self.stack_head().stack[a1] = self.stack_head().memory().get_u8(i)?;
            }
            Ok(Code::STORE_8) => {
                let i = self.stack_head().stack[a1];
                let v = self.stack_head().stack[a2] as u8;
                self.stack_head().memory().set_u8(i, v)?;
            }
            Ok(Code::LOAD_I) => {
                self.stack_head().stack[a1] = a2 + (a3 << 8);
            }


            Ok(Code::EXTEND) => {
                let offset = self.stack_head().stack[a1];
                self.stack_head().stack.extend(offset);
            }
            Ok(Code::EXTEND_I) => {
                self.stack_head().stack.extend(a1);
            }


            Ok(Code::SYSCALL) => {
                self.syscall(a1)?;
                return Ok(());
            }

            Err(()) => {
                print!("error");
            }
            Ok(_) => {} //TODO
        }
        self.stack_head().pc += 4;
        Ok(())
    }

    pub fn runs(&mut self, thread: *mut Thread) {
        self.stack.push(thread);
        while !self.stack.is_empty() {
            if let Err(signal) = self.run() {
                self.handle_error(signal);
            };
            self.handle_signal();
        }
    }

    fn handle_error(&mut self, error: Signal) {
        self.stack_head().status = ThreadStatus::Sleeping;
        self.stack.pop();
        if self.stack.is_empty() {
            // TODO : handle error
            println!("error: {:?}", error);
            return;
        }
        self.stack_head().pc += 4;
        self.stack_head().stack.push(error.into());
    } // TODO

    fn handle_signal(&mut self) {
        let Some(index) = self.stack.iter().position(|thread| {
            let thread = unsafe {
                &mut (**thread)
            };
            thread.signal.is_some()
        }) else {
            return;
        };

        let vec = self.stack.split_off(index + 2);
        self.stack_head().status = ThreadStatus::Interrupted(vec);
        self.stack.pop();
        self.stack_head().pc += 4;
    }
}
