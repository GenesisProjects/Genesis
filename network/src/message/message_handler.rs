use session::Session;
use message::defines::SocketMessage;

use std::collections::HashMap;

type SocketMessageCallback = fn(session: &mut Session, msg: &SocketMessage, name: String) -> bool;

pub struct SocketMessageHandler(HashMap<String, SocketMessageCallback>);

impl SocketMessageHandler {
    pub fn add_event_handler(&mut self, event: String, callback: SocketMessageCallback) {
        self.0.insert(event, callback);
    }

    pub fn remove_event_handler(&mut self, event: String, callback: SocketMessageCallback) {
        self.0.remove(&event);
    }
}