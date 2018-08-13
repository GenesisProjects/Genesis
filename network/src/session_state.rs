use gen_message::{Message, MESSAGE_CENTER};
use message::defines::*;
use message::message_handler::SocketMessageHandler;
use message::protocol::*;
use session::*;

pub trait EventRegister {
    fn add_handler(self) -> Self;
}

impl EventRegister for Session {
    fn add_handler(self) -> Self {
        let handler = self.handler.clone();
        let mut handler_ref = handler.borrow_mut();
        handler_ref.add_event_handler("BOOTSTRAP".to_string(), bootstrap_handler);
        handler_ref.add_event_handler("GOSSIP".to_string(), gossip_handler);
        handler_ref.add_event_handler("REJECT".to_string(), reject_handler);
        self
    }
}

fn bootstrap_handler(session: &mut Session, msg: &SocketMessage, name: String) -> bool {
    let args = msg.args();
    if !session.protocol().verify(&msg) {
        false
    } else {
        match session.status() {
            SessionStatus::Init => {
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
                session.set_status(SessionStatus::WaitGosship);
                // notify controller send gossip
                if let Some(token) = session.token() {
                    MESSAGE_CENTER.lock().unwrap().send(
                        name,
                        Message::new(token.0 as u16, "gossip".to_string())
                    );
                }
                true
            },
            _ => {
                // TODO: print cmd output here
                println!("Unavailable to process bootstrap right now");
                false
            }
        }
    }
}

fn gossip_handler(session: &mut Session, msg: &SocketMessage, name: String) -> bool {
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

fn reject_handler(session: &mut Session, msg: &SocketMessage, name: String) -> bool {
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