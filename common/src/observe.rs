use gen_message::*;

pub trait Observe {
    fn subscribe(&mut self);

    fn unsubscribe(&mut self);

    fn receive_async(&mut self) -> Option<Message>;

    fn receive_sync(&mut self) -> Message;
}