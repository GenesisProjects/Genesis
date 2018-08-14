pub extern crate common;
pub extern crate slab;

use ::common::observe::*;
use ::slab::Slab;

pub struct Pool<T> {
    name: String,
    slab: Slab<T>
}

impl<T> Pool<T> {
    pub fn add_subscriber<S>(&self, subscriber: &mut S) where S: Observe {
        subscriber.subscribe(self.name.to_owned())
    }
}