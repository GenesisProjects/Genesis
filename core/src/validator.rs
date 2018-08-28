use std::net::SocketAddr;
use common::address::Address;

#[derive(Debug, Clone)]
pub struct Validator {
    socket_addr: SocketAddr,
    account_addr: Address
}

impl Validator {
    pub fn new(socket_addr: SocketAddr, account_addr: Address) -> Self {
        Validator {
            socket_addr,
            account_addr
        }
    }

    pub fn socket_addr(&self) -> SocketAddr {
        self.socket_addr.clone()
    }

    pub fn account_addr(&self) -> Address {
        self.account_addr.clone()
    }
}