pub extern crate common;
pub extern crate gen_message;
pub extern crate slab;

use ::common::observe::*;
use ::slab::Slab;
use gen_message::*;

pub struct Pool<T> {
    name: String,
    channels: Vec<String>,
    slab: Slab<T>,
}

impl<T> Pool<T> {
    /// Init pool with capacity
    pub fn new(name: String, size: usize) -> Self {
        Pool {
            name: name,
            channels: vec![],
            slab: Slab::with_capacity(size)
        }
    }

    /// Add controller by channel name to notify
    pub fn add_channel(&mut self, name: String) {
        self.channels.push(name);
    }

    /// Remove controller by channel name
    pub fn remove_channel(&mut self, index: usize) {
        self.channels.remove(index);
    }

    /// Find channel index by name
    pub fn channel_index(&mut self, name: String) -> usize {
        self.channels
            .iter()
            .enumerate()
            .find(|r| r.1.to_owned() == name.to_owned())
            .unwrap()
            .0
    }

    /// Notify all channels if recieve a new transaction with message: "new_tx"
    pub fn notify_new_tx_recieved(&self) {
        self.channels
            .iter()
            .map(|ch| {
                MESSAGE_CENTER.lock()
                    .unwrap()
                    .send(
                        ch.to_string(),
                        Message::new(0, "new_tx".to_string()),
                    );
            });
    }


}