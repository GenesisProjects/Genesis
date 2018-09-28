extern crate gen_message;

pub mod observer;
pub mod thread;

use gen_message::Message;
use observer::*;
use thread::*;

pub use std::sync::mpsc::Receiver;

pub trait Processor {
    fn name(&self) -> String;

    fn status(&self) -> ThreadStatus;

    fn set_status(&self, status: ThreadStatus);

    fn reserve_msg_receiver(&mut self, recv: Receiver<Message>);

    fn receiver(&self) -> &Receiver<Message>;

    fn handle_msg(&mut self, msg: Message);
}

impl<T: Processor> Observer for T {
    fn channel_name(&self) -> String {
       self.name()
    }

    fn reserve_msg_receiver(&mut self, recv: Receiver<Message>) {
        self.reserve_msg_receiver(recv)
    }

    fn receiver(&self) -> &Receiver<Message> {
        self.receiver()
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
