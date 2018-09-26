//! Trait allows a thread running with a run loop
//!

use std::io::*;
use std::thread;
use std::sync::{Mutex, Arc, MutexGuard, mpsc::channel};
use std::time::Duration;

pub const LOOP_PERIOD: u64 = 100u64;


/// Thread status
#[derive(Copy, Clone)]
pub enum ThreadStatus {
    // Event loop is running
    Running,
    // Event loop has been killed
    Stop,
    // Event loop is on-hold
    Pause
}

/// Context cross thread reference
pub type ContextRef<T> = Arc<Mutex<T>>;


/// Thread context initializer trait
pub trait ThreadContextInitializer {
    fn init(name: String) -> ContextRef<Self>;
}

/// Thread information trait
pub trait ThreadInfo {
    /// get status
    fn status(&self) -> ThreadStatus;

    /// set status
    fn set_status(&mut self, status: ThreadStatus);

    /// thread name
    fn thread_name(&self) -> String;
}

/// Thread executor trait
pub trait ThreadExec {
    /// Run loop
    fn exec(&mut self) -> bool;
}

/// Thread service trait
pub trait ThreadService<ContextType> {
    /// Launch a thread.
    /// Return context reference
    fn launch(context_ref: ContextRef<ContextType>, stack_size: usize);

    /// Start the run loop
    fn start(&mut self);

    /// Pause the run loop
    fn pause(&mut self);

    /// Break the run loop
    fn stop(&mut self);
}

impl<ContextType: Send + 'static> ThreadService<ContextType> for ContextType
    where ContextType: ThreadContextInitializer + ThreadInfo + ThreadExec {
    fn launch(context_ref: ContextRef<ContextType>, stack_size: usize) {
        let mut context = context_ref.lock().unwrap();
        context.set_status(ThreadStatus::Pause);
        let name = context.thread_name();
        let thread_context_ref = context_ref.clone();

        // Spawn a thread to hold the run loop
        thread::Builder::new().stack_size(stack_size).name(name).spawn(move || {
            loop {
                let mut context = thread_context_ref.lock().unwrap();
                match context.status() {
                    ThreadStatus::Running => {
                        // exec run loop
                        context.exec();
                    },
                    ThreadStatus::Pause => {
                        // do nothing here
                    },
                    ThreadStatus::Stop => {
                        // break run loop
                        break;
                    }
                }
                thread::sleep(Duration::from_millis(LOOP_PERIOD));
            }
        });
    }

    fn start(&mut self) {
        self.set_status(ThreadStatus::Running);
    }

    fn pause(&mut self) {
        self.set_status(ThreadStatus::Pause);
    }

    fn stop(&mut self) {
        self.set_status(ThreadStatus::Stop);
    }
}