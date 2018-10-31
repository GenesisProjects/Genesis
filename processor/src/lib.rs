#[macro_use]
extern crate gen_message;

#[macro_use]
extern crate lazy_static;

pub mod observer;
pub mod thread;
pub mod thread_pool;

pub use gen_message::Message;
pub use observer::*;
pub use thread::*;
pub use std::sync::mpsc::Receiver;

use std::any::Any;
use std::sync::Arc;
use std::collections::HashMap;

/*lazy_static! {
    pub static ref THREAD_POOL: Mutex<HashMap<String, ContextRef<Processor>>> = {
        Mutex::new(ThreadPool::new(WORKER_NUM))
    };
}*/

/// The `Processor` trait is composed by `ThreadExec` trait and `Observer` trait.
/// A struct implement `Processor` will support the run loop task and thread safe message handling.
/// In order to make sure thread safe, we must use `launch` function to start a new thread
/// and obtain a `ContextRef` which is thread context handler that can be used in a different thread.
pub trait Processor: Any + Send {
    /// Processor name id.
    /// It must be unique.
    /// The message channel and thread use the name as the unique ID.
    fn name(&self) -> String;

    /// Description for this processor
    fn description(&self) -> String;

    /// The current thread status
    fn status(&self) -> ThreadStatus;

    /// Change thread status.
    fn set_status(&mut self, status: ThreadStatus);

    /// Get the current receiver
    fn receiver(&self) -> &Option<Receiver<Message>>;

    /// Set message receiver.
    /// Processor should store it somewhere in the current context.
    fn set_receiver(&mut self, recv: Receiver<Message>);

    /// Handle the incoming thread messages.
    fn handle_msg(&mut self, msg: Message);

    /// Before execute the run.
    fn pre_exec(&mut self) -> bool;

    /// Execute the run.
    fn exec(&mut self) -> bool;

    /// Span
    fn time_span(&self) -> u64;

    fn as_any(&self) -> &dyn Any;
}

impl<T: Processor> Observer for T {
    fn channel_name(&self) -> String {
       self.name()
    }

    fn reserve_msg_receiver(&mut self, recv: Receiver<Message>) {
        self.set_receiver(recv)
    }

    fn receiver(&self) -> &Option<Receiver<Message>> {
        self.receiver()
    }
}

impl <T: Processor> ThreadInfo for T {
    fn status(&self) -> ThreadStatus {
        self.status()
    }

    fn set_status(&mut self, status: ThreadStatus) {
        self.set_status(status);
    }

    fn thread_name(&self) -> String {
        self.name()
    }

    fn thread_update_time_span(&self) -> u64 {
        self.time_span()
    }
}

impl <T: Processor> ThreadExec for T {
    fn prepare(&mut self) {
        self.subscribe().expect(&format!("failed to subscribe channel: {:?}", self.name()));
    }

    fn pre_exec(&mut self) {
        self.pre_exec();
    }

    fn exec(&mut self) -> bool {
        self.exec()
    }

    fn post_exec(&mut self) {
        let msg = self.try_receive();
        if msg.is_some() {
            self.handle_msg(msg.unwrap());
        }
    }

    fn end(&mut self) {
        self.unsubscribe().expect(&format!("failed to unsubscribe channel: {:?}", self.name()));
    }
}
