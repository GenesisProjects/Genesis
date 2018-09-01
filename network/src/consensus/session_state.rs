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
    if let Some((prevote, account)) = session.protocol().verify_prevote(&msg) {
        let state_ref = session.state();
        let mut state = state_ref.borrow_mut();

        // Add prevote
        let has_consensus = state.add_prevote(&prevote);
        let mut full_propose = false;

        match state.get_propose(propose_hash) {
            Some(propose_state) => {
                if !propose_state.unknown_tnxs().is_empty() {
                    // Request transactions
                    let req = state.add_request(prevote.validator, RequestData::ProposeTransactions(prevote.propose_hash));
                    session.send_request(req.unwrap());
                } else {
                    full_propose = true;
                }
            }
            None => {
                // Request propose
                let req = state.add_request(prevote.validator, RequestData::Propose(prevote.propose_hash));
                session.send_request(req.unwrap());
            }
        }

        // Request prevotes if there are missing rounds
        if prevote.locked_round > state.locked_round() {
            let req = state.add_request(prevote.validator, RequestData::Prevotes(prevote.locked_round, prevote.propose_hash));
            session.send_request(req.unwrap());
        }


        if has_consensus && full_propose {
            // Todo Lock current state to the propose
        }

        true
    } else {
        false
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