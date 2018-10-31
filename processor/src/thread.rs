//! Trait allows a thread running with a run loop
//!

use thread_pool::ThreadPool;

use std::thread;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

const WORKER_NUM: usize = 32;

lazy_static! {
    pub static ref THREAD_POOL: Mutex<ThreadPool> = {
        Mutex::new(ThreadPool::new(WORKER_NUM))
    };
}

/// Thread status
#[derive(Debug, Copy, Clone)]
pub enum ThreadStatus {
    // Event loop is running
    Running,
    // Event loop has been killed
    Stop,
    // Event loop is on-hold
    Pause
}

/// Thread safe context reference.
pub struct ContextRef<T: ?Sized>(Arc<Mutex<T>>);

impl<T: ?Sized> ContextRef<T> {
    pub fn new_trait_obj_ref(trait_obj: Arc<Mutex<T>>) -> Self {
        ContextRef(trait_obj)
    }

    pub fn lock_trait_obj(&self) -> MutexGuard<T> {
        self.0.lock().unwrap()
    }
}

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

    /// time to sleep until next run
    fn thread_update_time_span(&self) -> u64;
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

    /// Called after a run loop
    fn end(&mut self);
}

/// Thread service trait
pub trait ThreadService<ContextType> {
    /// Launch a thread.
    /// Return context reference
    fn launch(self) -> ContextRef<ContextType>;

    /// Start the run loop
    fn start(&mut self);

    /// Pause the run loop
    fn pause(&mut self);

    /// Break the run loop
    fn stop(&mut self);
}

impl<ContextType> ThreadService<ContextType> for ContextType
    where ContextType: ThreadInfo + ThreadExec + Send + 'static {
    fn launch(self) -> ContextRef<ContextType> {
        let name = self.thread_name();
        let time_span = self.thread_update_time_span();
        let context_ref = ContextRef::new(self);
        let thread_context_ref = context_ref.clone();

        // Spawn a thread to hold the run loop
        THREAD_POOL.lock().unwrap().execute(move || {
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
                        thread::sleep_ms(500);
                    },
                    ThreadStatus::Stop => {
                        // break run loop
                        break;
                    }
                }
                thread::sleep(Duration::from_millis(time_span));
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