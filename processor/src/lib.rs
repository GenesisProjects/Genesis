extern crate gen_message;

pub mod observer;
pub mod thread;

use gen_message::Message;
use observer::*;
use thread::*;

use std::sync::Mutex;
pub use std::sync::mpsc::Receiver;

/// The max stack size for running process
pub const PROCESSOR_STACK_SIZE: usize = 4 * 1024 * 1024 * 1024;

/// The `Processor` trait is composed by `ThreadExec` trait and `Observer` trait.
/// A struct implement `Processor` will support the run loop task and thread safe message handling.
/// In order to make sure thread safe, we must use `launch` function to start a new thread
/// and obtain a `ContextRef` which is thread context handler that can be used in a different thread.
pub trait Processor {
    /// Processor name id.
    /// It must be unique.
    /// The message channel and thread use the name as the unique ID.
    fn name(&self) -> String;

    /// Description for this processor
    fn description(&self) -> String;

    /// The current thread status
    fn status(&self) -> ThreadStatus;

    /// Change thread status.
    fn set_status(&self, status: ThreadStatus);

    /// Set message receiver.
    /// Processor should store it somewhere in the current context.
    fn set_receiver(&mut self, recv: Receiver<Message>);

    /// Get the current receiver
    fn receiver(&self) -> &Receiver<Message>;

    /// Handle the incoming thread messages.
    fn handle_msg(&mut self, msg: Message);
}

impl<T: Processor> Observer for T {
    fn channel_name(&self) -> String {
       self.name()
    }

    fn reserve_msg_receiver(&mut self, recv: Receiver<Message>) {
        self.set_receiver(recv)
    }

    fn receiver(&self) -> &Receiver<Message> {
        self.receiver()
    }
}

impl <T: Processor> ThreadInfo for T {
    #[inline]
    fn status(&self) -> ThreadStatus {
        self.status()
    }

    #[inline]
    fn set_status(&mut self, status: ThreadStatus) {
        self.set_status(status)
    }

    #[inline]
    fn thread_name(&self) -> String {
        self.name()
    }
}

impl <T: Processor> ThreadExec for T {
    #[inline]
    fn prepare(&mut self) {
        self.subscribe().expect(&format!("failed to subscribe channel: {:?}", self.name()));
    }

    #[inline]
    fn pre_exec(&mut self) {
        unimplemented!()
    }

    #[inline]
    fn exec(&mut self) -> bool {
        unimplemented!()
    }

    #[inline]
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
