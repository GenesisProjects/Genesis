extern crate common;
extern crate num;

use self::common::address::Address;
use self::common::key::Signature;
use self::common::hash::{ Hash, SHA256Hashable };
use self::num::bigint::BigInt;
use self::num::Zero;

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
    txdata: TransactionBody,

    // caches
    hash: Option<Hash>,
    size: Option<u32>,
    from: Option<Address>,
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
            txdata: TransactionBody {
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
            hash: None,
            size: None,
            from: None,
        })
    }
}

/// sign transaction
impl SHA256Hashable for Transaction {
    fn serialized_data(&self) -> Vec<u8> {
        let r = vec![1,2,3];
        return r;
    }
}