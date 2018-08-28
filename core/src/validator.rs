use std::net::SocketAddr;
use common::address::Address;

#[derive(Debug, Clone)]
pub struct Validator {
    validator_id: ValidatorId,
    socket_addr: SocketAddr,
    account_addr: Address
}

/// Validators identifier.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValidatorId(pub u16);

impl ValidatorId {
    /// Returns zero value of the validator id.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// let id = ValidatorId::zero();
    /// assert_eq!(0, id.0);
    /// ```
    pub fn zero() -> Self {
        ValidatorId(0)
    }
}

impl Validator {
    pub fn new(socket_addr: SocketAddr, account_addr: Address, validator_id: ValidatorId) -> Self {
        Validator {
            validator_id,
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