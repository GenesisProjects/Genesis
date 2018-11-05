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

        let has_unknown_tnxs = match state.add_propose(&propose) {
            Ok(propose_state) => propose_state.has_unknown_tnxs(),
            Err(err) => {
                return false;
            }
        };

        let propose_hash = propose.hash();

        // Remove request info
        state.remove_request(&RequestData::Propose(propose_hash));

        if has_unknown_tnxs {
            session.send_request(RequestData::ProposeTransactions(propose_hash));

        } else {
            state.handle_full_propose(propose_hash, propose.round);
        }
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

        let req = match state.get_propose(&prevote.propose_hash) {
            Some(propose_state) => {
                if !propose_state.unknown_tnxs().is_empty() {
                    // Request transactions
                    Some(RequestData::ProposeTransactions(prevote.propose_hash))
                } else {
                    full_propose = true;
                    None
                }
            }
            None => {
                // Request propose
                Some(RequestData::Propose(prevote.propose_hash))
            }
        };

        if let Some(req_data) = req {
            state.add_request(prevote.validator, req_data.clone());
            session.send_request(req_data);
        }

        // Request prevotes if there are missing rounds
        if prevote.locked_round > state.locked_round() {
            let req = state.add_request(prevote.validator, RequestData::Prevotes(prevote.locked_round, prevote.propose_hash));
            session.send_request(req.unwrap());
        }


        if has_consensus && full_propose {
            // Todo Lock current state to the propose
            state.handle_majority_prevotes(prevote.round, &prevote.propose_hash);
        }

        true
    } else {
        false
    }
}

fn precommit_handler(session: &mut Session, msg: &SocketMessage, name: String) -> bool {
    let args = msg.args();
    if let Some((precommit, account)) = session.protocol().verify_precommit(&msg) {
        let state_ref = session.state();
        let mut state = state_ref.borrow_mut();

        // Add prevote
        let has_consensus = state.add_precommit(&precommit);

        // Request propose
        if state.get_propose(&precommit.propose_hash).is_none() {
            session.send_request(RequestData::Propose(precommit.propose_hash));
        }

        // Request prevotes
        if precommit.round > state.locked_round() {
            session.send_request(RequestData::Prevotes(precommit.round, precommit.propose_hash));
        }

        // Has majority precommits
        if has_consensus {
            let has_unknown_tnxs = state.handle_majority_precommits(precommit.round, &precommit.propose_hash, &precommit.block_hash);
            session.send_request(RequestData::ProposeTransactions(precommit.propose_hash));
        }

        true
    } else {
        false
    }
}