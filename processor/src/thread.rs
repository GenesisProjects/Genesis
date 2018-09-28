//! Trait allows a thread running with a run loop
//!

use std::thread;
use std::sync::{Arc, Mutex, MutexGuard};
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
pub struct ContextRef<T>(Arc<Mutex<T>>);

impl<T> ContextRef<T> {
    pub fn new(context: T) -> Self {
        ContextRef(Arc::new(Mutex::new(context)))
    }

    pub fn lock(&self) -> MutexGuard<T> {
        self.0.lock().unwrap()
    }
}

impl<T> Clone for ContextRef<T> {
    fn clone(&self) -> Self {
        ContextRef(self.0.clone())
    }
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
    /// Called before a run loop
    fn prepare(&mut self);

    /// Called at begin of the loop
    fn pre_exec(&mut self);

    /// Exec
    fn exec(&mut self) -> bool;

    /// Called at end of the loop
    fn post_exec(&mut self);
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

impl<ContextType> ThreadService<ContextType> for ContextType
    where ContextType: ThreadInfo + ThreadExec + Send + 'static {
    fn launch(context_ref: ContextRef<ContextType>, stack_size: usize) {
        let context = context_ref.lock();
        let name = context.thread_name();
        let thread_context_ref = context_ref.clone();

        // Spawn a thread to hold the run loop
        thread::Builder::new().stack_size(stack_size).name(name).spawn(move || {
            let mut context = thread_context_ref.lock();
            context.set_status(ThreadStatus::Pause);
            context.prepare();
            loop {
                match context.status() {
                    ThreadStatus::Running => {
                        // exec run loop
                        context.pre_exec();
                        context.exec();
                        context.post_exec();
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
        }).unwrap();
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