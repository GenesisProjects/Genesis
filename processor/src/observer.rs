use super::gen_message::{Message, MessageCenterError, MESSAGE_CENTER};
use std::sync::mpsc::Receiver;

/// Observer trait will permit an object to subscribe/unsubcribe/receive message from a message center
pub trait Observer {
    /// The unique channel name.
    /// Each Observer should return a unique channel name
    /// or will lead to `MessageCenterError` when it subscribes a message center.
    fn channel_name(&self) -> String;

    /// Reserve the receiving end of the message channel.
    /// Observer could receive the message by using it.
    /// Only one copy of Receiver is allowed.
    fn reserve_msg_receiver(&mut self, recv: Receiver<Message>);

    /// Get receiver ref
    fn receiver(&self) -> &Receiver<Message>;
}

/// To let the object receive thread messages.
/// It's non-blocking.
pub trait Receiving {
    /// Try to receive a message in non-blocking way.
    /// If the channel is hangup or get exception , return `None`.
    fn try_receive(&self) -> Option<Message>;
}

impl<T: Observer> Receiving for T {
    fn try_receive(&self) -> Option<Message> {
        match self.receiver().try_recv() {
            Ok(msg) => Some(msg),
            Err(_) => None
        }
    }
}

/// Observer service trait
pub trait ObserverService {
    /// Subscribe a message center, update receiver.
    fn subscribe(&mut self) -> Result<(), MessageCenterError>;
    /// Unsubscribe a message center.
    fn unsubscribe(&mut self) -> Result<(), MessageCenterError>;
}

impl<T: Observer> ObserverService for T {
    fn subscribe(&mut self) -> Result<(), MessageCenterError> {
        let mut guard = MESSAGE_CENTER.lock().unwrap();
        guard.subscribe(self.channel_name()).and_then(|recv| {
            Ok(self.reserve_msg_receiver(recv))
        })
    }

    fn unsubscribe(&mut self) -> Result<(), MessageCenterError> {
        let mut guard = MESSAGE_CENTER.lock().unwrap();
        guard.unsubscribe(self.channel_name())
    }
}