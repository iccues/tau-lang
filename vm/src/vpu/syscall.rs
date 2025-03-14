use library::instruction::opcode::syscall::Syscall;
use library::signal::SignalResult;

use crate::process::thread::thread_status::ThreadStatus;
use crate::process::thread::Thread;
use super::Vpu;

impl Vpu {
    pub(super) fn syscall(&mut self, a1: usize) -> SignalResult<()> {
        let syscall: Result<Syscall, ()> = (a1 as u8).try_into();

        match syscall {
            // Call
            Ok(Syscall::CALL_P) => {
                self.call_p()?;
                return Ok(());
            }
            Ok(Syscall::RET_P) => {
                self.ret_p()?;
                return Ok(());
            }

            // // Signal
            // Ok(Syscall::signal) => {
            //     let thread = self.stack_head().stack[0];
            //     let signal = self.stack_head().stack[1].into();
            //     self.stack_head().process().threads.get_thread(thread)?.signal(signal);
            // }

            // Interrupt
            Ok(Syscall::INT) => {
                let thread = self.stack_head().stack.pop()?;
                let signal = self.stack_head().stack.pop()?.into();
                self.stack_head().process().threads.get_thread(thread)?.signal(signal);
            }
            Ok(Syscall::INT_RET) => {
                self.int_ret()?;
            }

            // Memory
            Ok(Syscall::SET_PAGE) => {
                let r0 = self.stack_head().stack.pop()?;
                let r1 = self.stack_head().stack.pop()?;
                let r2 = self.stack_head().stack.pop()?;

                self.stack_head().process().
                get_child(r0)?.
                memory().set_page(r1, r2);
            }
            Ok(Syscall::DEL_PAGE) => {
                let r0 = self.stack_head().stack.pop()?;
                let r1 = self.stack_head().stack.pop()?;

                self.stack_head().process().
                get_child(r0)?.
                memory().del_page(r1);
            }

            // Process
            Ok(Syscall::NEW_CHILD) => {
                let pid = self.stack_head().process().new_child()?;
                self.stack_head().stack.push(pid);
            }
            Ok(Syscall::DEL_CHILD) => {
                let index = self.stack_head().stack.pop()?;
                self.stack_head().process().del_child(index)?;
            }

            // Threads
            Ok(Syscall::NEW_THREAD) => {
                let thread = self.stack_head().process().threads.new_thread()?;
                self.stack_head().stack.push(thread);
            }
            Ok(Syscall::DEL_THREAD) => {
                let index = self.stack_head().stack.pop()?;
                self.stack_head().process().threads.del_thread(index)?;
            }

            // IO
            Ok(Syscall::PRINT) => {
                print!("{}", self.stack_head().stack[0]);
            }
            Ok(Syscall::PRINT_C) => {
                print!("{}", self.stack_head().stack[0] as u8 as char);
            }

            Err(()) => {}
        }
        self.stack_head().pc += 4;
        Ok(())
    }

    fn call_p(&mut self) -> SignalResult<()> {
        let child_index = self.stack_head().stack.pop()?;
        let thread_index = self.stack_head().stack.pop()?;
        let args = self.stack_head().stack.pop_args()?;

        self.push_thread(child_index, thread_index)?;
        self.stack_head().pc += 4;

        self.stack_head().status = ThreadStatus::Running;

        self.stack_head().stack.push_args(&args);

        Ok(())
    }

    fn int_ret(&mut self) -> SignalResult<()> {
        let child_index = self.stack_head().stack.pop()?;
        let thread_index = self.stack_head().stack.pop()?;

        self.push_thread(child_index, thread_index)?;

        let threads = self.stack_head().status.is_interrupted()?.clone();
        self.stack_head().status = ThreadStatus::Running;
        self.stack.extend_from_slice(&threads);

        Ok(())
    }

    fn push_thread(&mut self, child_index: usize, thread_index: usize) -> SignalResult<()> {
        let ptr: *mut Thread = self
            .stack_head()
            .process()
            .get_child(child_index)?
            .threads
            .get_thread(thread_index)?;

        self.stack.push(ptr);

        Ok(())
    }

    fn ret_p(&mut self) -> SignalResult<()> {
        self.stack_head().status = ThreadStatus::Sleeping;

        let args = self.stack_head().stack.pop_args()?;

        self.stack_head().status = ThreadStatus::Sleeping;

        self.stack.pop();
        if self.stack.is_empty() {
            self.ret_call_0(&args);
            return Ok(());
        }

        self.stack_head().stack.push_args(&args);

        self.stack_head().pc += 4;
        Ok(())
    }
}
