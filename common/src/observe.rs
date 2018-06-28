use std::sync::{Arc, Mutex};

use gen_message::*;

pub trait Observe {
    fn subscribe(&mut self);

    fn unsubscribe(&mut self);

    fn send(&mut self);

    fn receive(&mut self);
}