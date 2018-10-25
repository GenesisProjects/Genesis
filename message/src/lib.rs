//! This crate allows Genesis send message async between different thread.
//!
//! ```
//!
//! let ch_name = "test".to_string();
//! subscribe!(ch_name);
//! unsubscribe!(ch_name);
//!
//! ```

#[macro_use]
extern crate lazy_static;

pub mod defines;

use std::sync::Mutex;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MessageCenterError(String);

/// Message center singleton
lazy_static! {
    pub static ref MESSAGE_CENTER: Mutex<MessageCenter> = {
        Mutex::new(MessageCenter::new())
    };
}

#[macro_export]
macro_rules! notify {
    ($name: expr, $msg: expr) => {{
        let guard = MESSAGE_CENTER.lock().unwrap();
        guard.notify($name, $msg)
    }};
}

#[macro_export]
macro_rules! subscribe {
    ($name: expr) => {{
        let mut guard = MESSAGE_CENTER.lock().unwrap();
        guard.subscribe($name)
    }};
}

#[macro_export]
macro_rules! unsubscribe {
    ($name: expr) => {{
        let mut guard = MESSAGE_CENTER.lock().unwrap();
        guard.unsubscribe($name)
    }};
}

/// Inter-thread message
#[derive(Debug, Clone)]
pub struct Message {
    msg: String,
    id: u32,
    body: Vec<u8>
}

impl Message {
    pub fn new(msg: String, id: u32, body: Vec<u8>) -> Self {
        Message {
            msg: msg,
            id: id,
            body: body
        }
    }

    pub fn msg(&self) -> String {
        self.msg.clone()
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn body(&self) -> Vec<u8> {
        self.body.clone()
    }
}

pub struct MessageCenter {
    channel_map: HashMap<String, Sender<Message>>
}

impl MessageCenter {
    pub fn new() -> Self {
        MessageCenter {
            channel_map: HashMap::new()
        }
    }

    /// Subscribe a channel by name.
    /// Return the receiving terminal for caller.
    pub fn subscribe(&mut self, name: String) -> Result<Receiver<Message>, MessageCenterError> {
        let existed = self.channels_exist_by_name(name.to_owned());
        if existed {
            Err(MessageCenterError("Channel already existed!".into()))
        } else {
            let (sender, receiver) = channel();
            self.channel_map.insert(name, sender);
            Ok(receiver)
        }
    }

    pub fn unsubscribe(&mut self, name: String) -> Result<(), MessageCenterError> {
        let existed = self.channels_exist_by_name(name.to_owned());
        if existed {
            self.channel_map.remove(&name);
            Ok(())
        } else {
            Err(MessageCenterError("Can not find the channel.".into()))
        }
    }

    pub fn notify(&self, name: String, msg: Message) -> Result<(), MessageCenterError> {
        let existed = self.channels_exist_by_name(name.to_owned());
        if !existed {
            Err(MessageCenterError("Can not find the channel.".into()))
        } else {
            let sender = self.channel_map.get(&name).unwrap().clone();
            sender.send(msg).map_err(|e| {
                MessageCenterError(e.to_string())
            })
        }
    }

    pub fn channels_exist_by_name(&self, name: String) -> bool {
        let sender = self.channel_map.get(&name);
        match sender {
            Some(_) => true,
            None => false
        }
    }
}