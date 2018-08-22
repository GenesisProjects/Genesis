#![cfg_attr(all(not(test), stage0), feature(float_internals))]

//! This module allows Genesis to send `SocketMessage`.
//! `SocketMessage` will be serialized cto json format by using crate [serde](https://crates.io/crates/serde).
//! Please implement your own **protocal** to build & verify `SocketMessage`.

pub mod defines;
pub mod message_handler;