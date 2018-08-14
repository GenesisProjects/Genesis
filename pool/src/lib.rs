pub extern crate common;
pub extern crate slab;

use ::common::observe::*;
use ::slab::Slab;

pub struct Pool<T> {
    name: String,
    channels: Vec<String>,
    slab: Slab<T>
}

impl<T> Pool<T> {
    pub fn add_channel(&mut self, name: String) {
        self.channels.push(name);
    }

    pub fn remove_channel(&mut self, index: usize) {
        self.channels.remove(index);
    }
    pub fn channel_index(&mut self, name: String) -> usize {
        self.channels
            .iter()
            .enumerate()
            .find(|r| r.1.to_owned() == name.to_owned())
            .unwrap()
            .0
    }
}