use message::message_handler::SocketMessageHandler;
use session::*;

pub trait EventRegister {
    fn init_handler(&mut self);
}

impl EventRegister for Session {
    fn init_handler(&mut self) {
        unimplemented!()
    }
}

/*
match event {
            "BOOTSTRAP" => {
                if !self.protocol.verify(&msg) {
                    false
                } else {
                    match self.status {
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
                            self.table = PeerTable::new_with_hosts(hosts);
                            self.status = SessionStatus::WaitGosship;
                            // notify controller send gossip
                            if let Some(token) = self.token.clone() {
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
            },
            "GOSSIP" => {
                if !self.protocol.verify(&msg) {
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
                    self.table = PeerTable::new_with_hosts(hosts);
                    self.status = SessionStatus::WaitBlockInfoRequest;
                    true
                }
            },
            "REJECT" => {
                if !self.protocol.verify(&msg) {
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
                    match self.status {
                        SessionStatus::Init | SessionStatus::WaitBlockInfoRequest => {
                            self.status = SessionStatus::ConnectionReject;
                            true
                        },
                        _ => false
                    }
                }
            },
            _ => false
        }
*/