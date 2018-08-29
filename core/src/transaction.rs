use chrono::{DateTime, Utc};
use common::address::Address;
use common::key::Signature;
use num::bigint::BigInt;
use num::Zero;
use rlp::RLPSerialize;
use rlp::types::*;

///
///
///
#[derive(Clone)]
pub struct TransactionBody {
    timestamp: DateTime<Utc>,
    sender: Address,
    recipient: Address,
    amount: BigInt,
    sig: Option<Signature>,
}

///
///
///
#[derive(Clone)]
pub struct Transaction {
    tx_body: TransactionBody
}

impl Transaction {
    pub fn new(nonce: u64,
               from: Address,
               to: Address,
               amount: Option<BigInt>,
               data: &Vec<u8>) -> Self {
        Transaction {
            tx_body: TransactionBody {
                timestamp: Utc::now(),
                sender: from,
                recipient: to,
                amount: match amount {
                    Some(v) => v,
                    None => BigInt::zero()
                },
                sig: None,
            },
        }
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.tx_body.timestamp.clone()
    }
}

impl RLPSerialize for Transaction {
    fn serialize(&self) -> Result<RLP, RLPError> {
        unimplemented!()
    }

    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_transaction() {}
}