use common::address::Address;
use common::key::Signature;
use num::bigint::BigInt;
use num::Zero;
use rlp::RLPSerialize;
use rlp::types::*;

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
        unimplemented!()
    }

    fn deserialize(rlp: &RLP) -> Result<Self, RLPError> {
        unimplemented!()
    }
}

# [cfg(test)]
mod tests {
    # [test]
    fn test_transaction() {}
}