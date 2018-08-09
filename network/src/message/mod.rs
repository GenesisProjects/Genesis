#![cfg_attr(all(not(test), stage0), feature(float_internals))]

pub mod defines;
pub mod message_handler;
pub mod protocol;