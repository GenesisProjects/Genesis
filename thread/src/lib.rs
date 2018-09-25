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
    fn init(name: String) -> Self;
}

/// Thread information trait
pub trait ThreadInfo {
    /// get status
    fn status(&self) -> ThreadStatus;

    /// set status
    fn set_status(&mut self, status: ThreadStatus);

    /// stack size
    fn name() -> String;
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
    fn launch(name: String, stack_size: usize) -> ContextRef<ContextType>;

    /// Start the run loop
    fn start(&mut self);

    /// Pause the run loop
    fn pause(&mut self);

    /// Break the run loop
    fn stop(&mut self);
}

impl<ContextType: Send + 'static> ThreadService<ContextType> for ContextType
    where ContextType: ThreadContextInitializer + ThreadInfo + ThreadExec {
    fn launch(name: String, stack_size: usize) -> ContextRef<ContextType> {
        let mut context = ContextType::init(name.to_owned());
        context.set_status(ThreadStatus::Pause);
        let context_ref = Arc::new(Mutex::new(context));
            //ContextRef::wrap(context);
        let thread_reserved_context_ref = context_ref.clone();

        // Spawn a thread to hold the run loop
        thread::Builder::new().stack_size(stack_size).name(name.to_owned()).spawn(move || {
            loop {
                let mut context = thread_reserved_context_ref.lock().unwrap();
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
        context_ref
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