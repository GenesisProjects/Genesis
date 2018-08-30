use gen_message::{Message, MESSAGE_CENTER};
use socket::message::defines::*;
use socket::message::message_handler::*;

use super::session::*;
use super::protocol::*;

impl EventRegister for Session {
    fn add_handler(self) -> Self {
        let handler = self.handler.clone();
        let mut handler_ref = handler.borrow_mut();
        handler_ref.add_event_handler("PROPOSE".to_string(), propose_handler);
        handler_ref.add_event_handler("PREVOTE".to_string(), prevote_handler);
        handler_ref.add_event_handler("PRECOMMIT".to_string(), precommit_handler);
        self
    }
}

fn propose_handler(session: &mut Session, msg: &SocketMessage, name: String) -> bool {
    let args = msg.args();
    if let Some((propose, account)) = session.protocol().verify_propose(&msg) {
        let state_ref = session.state();
        let mut state = state_ref.borrow_mut();

        if account != state.get_validator_key(propose.validator).unwrap() {
            return false;
        }

        if propose.prev_hash != state.last_hash() {
            return false;
        }

        if propose.validator != state.leader(propose.round) {
            return false;
        }

        state.add_propose(&propose);
        // Todo add propose and check if there are unknown tnxs
        true
    } else {
        false
    }
}

fn prevote_handler(session: &mut Session, msg: &SocketMessage, name: String) -> bool {
    let args = msg.args();
    if !session.protocol().verify(&msg) {
        false
    } else {
        let state = session.state();
        // Todo Check leader + unknown tnxs + handle propose
        true
    }
}

fn precommit_handler(session: &mut Session, msg: &SocketMessage, name: String) -> bool {
    let args = msg.args();
    if !session.protocol().verify(&msg) {
        false
    } else {
        let state = session.state();
        // Todo Check leader + unknown tnxs + handle propose
        true
    }
}