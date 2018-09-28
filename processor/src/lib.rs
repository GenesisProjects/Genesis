extern crate gen_message;
#[macro_use]
extern crate lazy_static;

pub mod observer;
pub mod thread;

use gen_message::Message;
use observer::*;
use thread::*;

use std::boxed::Box;
use std::collections::HashMap;
use std::sync::Mutex;
pub use std::sync::mpsc::Receiver;

/*lazy_static! {
    static ref HASHES: Mutex<HashMap<&'static str, Mutex<Box<Processor + 'static>>>> = {
        Mutex::new(HashMap::new())
    };
}*/

pub const PROCESSOR_STACK_SIZE: usize = 40 * 1024 * 1024;

pub trait Processor {
    fn name(&self) -> String;

    fn description(&self) -> String;

    fn status(&self) -> ThreadStatus;

    fn set_status(&self, status: ThreadStatus);

    fn set_receiver(&mut self, recv: Receiver<Message>);

    fn receiver(&self) -> &Receiver<Message>;

    fn handle_msg(&mut self, msg: Message);
}

pub struct ProcessorManager {
    processors: HashMap<&'static str, Box<Processor + 'static>>
}

impl ProcessorManager {
    fn register<T>(name: String, context: T)
        where T: Processor +'static {
        let context_ref = ContextRef::new(context);
        context.launch(PROCESSOR_STACK_SIZE);
    }
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
