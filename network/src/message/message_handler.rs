use session::Session;
use message::defines::SocketMessage;

use std::collections::HashMap;

pub type SocketMessageCallback = fn(session: &mut Session, msg: &SocketMessage, name: String) -> bool;

#[derive(Clone)]
pub struct SocketMessageHandler(HashMap<String, SocketMessageCallback>);

impl SocketMessageHandler {
    pub fn new() -> Self {
        SocketMessageHandler(HashMap::new())
    }

    pub fn add_event_handler(&mut self, event: String, callback: SocketMessageCallback) {
        self.0.insert(event, callback);
    }

    pub fn remove_event_handler(&mut self, event: String) {
        self.0.remove(&event);
    }

    pub fn process_event(
        &mut self,
        event: String,
        session: &mut Session,
        msg: &SocketMessage) -> bool {
        if let Some(f) = self.0.get(&event) {
            f(session, msg, event.to_owned())
        } else {
            false
        }
    }
}