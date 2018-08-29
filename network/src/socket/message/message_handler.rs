//! Handle message with callback functions.
//! # Example:
//! ```ignore
//! use gen_network::socket::message::message_handler::*;
//!
//! // Define a test session struct, this will be used as a parameter into the callback function.
//! struct TestSession;
//!
//! // Define a test struct which own a `SocketMessageHandler` instance.
//! struct Test {
//!      pub handler: SocketMessageHandler<TestSession>
//! }
//!
//! // Implement `EventRegister` for the `Test` struct
//! impl EventRegister for Test {
//!     fn add_handler(self) -> Self {
//!         self.handler.add_event_handler("AnyEvent".to_string(), any_event_handler);
//!     }
//! }
//!
//! // Initialize test object, inject callbacks.
//! let test = Test { handler: SocketMessageHandler::new() };
//! let mut test_with_handler = test.add_handler();
//!
//! // Define callback functions
//! fn any_event_handler(session: &mut TestSession, msg: &SocketMessage, name: String) -> bool {
//!     println!("recv: {:?}", msg);
//!     true
//! }
//!
//! // Invoke the callback
//! test.handler.process_event();
//! ```

use super::defines::SocketMessage;
use std::collections::HashMap;

/// Callback function.
/// ## Params:
/// *session - caller
/// *name - caller name, generally the channel name of the caller.
///
/// ## Return
/// return success or failed
pub type SocketMessageCallback<T> = fn(session: &mut T, msg: &SocketMessage, name: String) -> bool;

/// Implement this trait allow struct to add `SocketMessageCallback` into it's `SocketMessageHandler`
pub trait EventRegister {
    fn add_handler(self) -> Self;
}

/// Add/Del `SocketMessageCallback`.
/// Invoke `SocketMessageCallback` by `event` string.
#[derive(Clone)]
pub struct SocketMessageHandler<T>(HashMap<String, SocketMessageCallback<T>>);

impl<T> SocketMessageHandler<T> {
    pub fn new() -> Self {
        SocketMessageHandler(HashMap::new())
    }

    /// Add a `SocketMessageCallback` with an `event` name.
    pub fn add_event_handler(&mut self, event: String, callback: SocketMessageCallback<T>) {
        self.0.insert(event, callback);
    }

    /// Remove a `SocketMessageCallback` by the `event` name.
    pub fn remove_event_handler(&mut self, event: String) {
        self.0.remove(&event);
    }

    /// Process an event.
    /// Select the relative `SocketMessageCallback` to invoke.
    pub fn process_event(
        &mut self,
        event: String,
        session: &mut T,
        msg: &SocketMessage) -> bool {
        if let Some(f) = self.0.get(&event) {
            f(session, msg, event.to_owned())
        } else {
            false
        }
    }
}