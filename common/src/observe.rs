use std::sync::{Arc, Mutex};

use gen_message::*;

pub trait Observe {
    fn subscribe(&mut self, name: String);

    fn unsubscribe(&mut self, uid: String);

    fn send(&mut self, msg: Message);

    fn receive_async(&mut self) -> Option<Message>;

    fn receive_sync(&mut self) -> Message;
}