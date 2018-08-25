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
    if !session.protocol().verify(&msg) {
        false
    } else {
        // Check leader
        if msg.validator() != self.state.leader(msg.round()) {
            error!(
                "Wrong propose leader detected: actual={}, expected={}",
                msg.validator(),
                self.state.leader(msg.round())
            );
            return;
        }

        trace!("Handle propose");

        let snapshot = self.blockchain.snapshot();
        let schema = Schema::new(snapshot);
        //TODO: Remove this match after errors refactor. (ECR-979)
        let has_unknown_txs =
            match self.state
                .add_propose(msg, &schema.transactions(), &schema.transactions_pool())
                {
                    Ok(state) => state.has_unknown_txs(),
                    Err(err) => {
                        warn!("{}, msg={:?}", err, msg);
                        return;
                    }
                };

        let hash = msg.hash();

        // Remove request info
        let known_nodes = self.remove_request(&RequestData::Propose(hash));

        if has_unknown_txs {
            trace!("REQUEST TRANSACTIONS");
            self.request(RequestData::ProposeTransactions(hash), from);

            for node in known_nodes {
                self.request(RequestData::ProposeTransactions(hash), node);
            }
        } else {
            self.handle_full_propose(hash, msg.round());
        }
    }
}

fn prevote_handler(session: &mut Session, msg: &SocketMessage, name: String) -> bool {
    let args = msg.args();
    if !session.protocol().verify(&msg) {
        false
    } else {
        let slice = &args[3 .. ];
        let mut hosts: Vec<String> = vec![];
        for arg in slice {
            match arg {
                &SocketMessageArg::String { ref value } => {
                    //TODO: make port configurable
                    hosts.push(value.clone())
                }
                _ => ()
            };
        }
        session.set_table(PeerTable::new_with_hosts(hosts));
        session.set_status(SessionStatus::WaitBlockInfoRequest);
        true
    }
}

fn precommit_handler(session: &mut Session, msg: &SocketMessage, name: String) -> bool {
    let args = msg.args();
    if !session.protocol().verify(&msg) {
        false
    } else {
        match &args[3] {
            &SocketMessageArg::String { ref value } => {
                println!("Rejected!");
            },
            _ => {
                return false;
            }
        }
        match session.status() {
            SessionStatus::Init | SessionStatus::WaitBlockInfoRequest => {
                session.set_status(SessionStatus::ConnectionReject);
                true
            },
            _ => false
        }
    }
}