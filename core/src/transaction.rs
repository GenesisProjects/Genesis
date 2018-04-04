extern crate common;
extern crate num;
extern crate rlp;

use self::common::address::Address;
use self::common::key::Signature;
use self::common::hash::{ Hash, SerializableAndSHA256Hashable };
use self::num::bigint::BigInt;
use self::num::Zero;
use self::rlp::RLPSerialize;
use self::rlp::types::*;

use std::marker::PhantomData;

///
///
///
pub struct TransactionBody {
    account_nounce: u64,
    gas_price: BigInt,
    gas_limit: u64,
    sender: Address,
    recipient: Address,
    amount: BigInt,
    payload: Vec<u8>,
    sig: Option<Signature>
}

///
///
///
pub struct Transaction {
    tx_body: TransactionBody
}

impl Transaction {
    pub fn new(nonce: u64,
               from: Address,
               to: Address,
               amount: Option<BigInt>,
               gas_limit: u64,
               gas_price: Option<BigInt>,
               data: &Vec<u8>) -> Box<Self> {
        Box::new(Transaction {
            tx_body: TransactionBody {
                account_nounce: nonce,
                gas_price: match gas_price {
                    Some(v) => v,
                    None => BigInt::zero()
                },
                gas_limit: gas_limit,
                sender: from,
                recipient: to,
                amount: match amount {
                    Some(v) => v,
                    None => BigInt::zero()
                },
                payload: data.to_vec(),
                sig: None
            },
        })
    }
}

impl RLPSerialize for Transaction {
    fn serialize(&self) -> Result<RLP, RLPError> {
        Err(RLPError::RLPErrorUnknown)
    }

    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        Err(RLPError::RLPErrorUnknown)
    }
}

# [cfg(test)]
mod tests {
    # [test]
    fn test_transaction() {}
}