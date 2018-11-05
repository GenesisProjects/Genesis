use chrono::{DateTime, Utc};
use common::address::Address;
use common::hash::{Hash, SerializableAndSHA256Hashable};
use common::key::{Signature, KeyPair, KeyPairOp};
use num::Zero;
use rlp::RLPSerialize;
use rlp::types::*;

///
///
///
#[derive(Clone)]
pub struct Transaction {
    hash: Option<Hash>,
    timestamp: DateTime<Utc>,
    nonce: u64,
    sender: Address,
    recipient: Address,
    amount: u64,
    signature: Option<Signature>,
}

impl Transaction {
    pub fn new(
        nonce: u64,
        from: Address,
        to: Address,
        amount: u64
    ) -> Self {
        Transaction {
            hash: None,
            timestamp: Utc::now(),
            nonce: nonce,
            sender: from,
            recipient: to,
            amount: amount,
            signature: None
        }
    }

    pub fn gen_hash(&self) -> Hash {
        let (hash, _) = self.encrype_sha256().unwrap();
        hash
    }

    pub fn hash(&self) -> Hash {
        self.hash.clone().unwrap()
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp.clone()
    }

    pub fn sender(&self) -> Address {
        self.sender.clone()
    }

    pub fn recipient(&self) -> Address {
        self.recipient.clone()
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    pub fn check(&self) -> bool {
        self.hash.is_some()
            && self.signature.is_some()
            && KeyPair::verify_sig(
                &self.sender,
                &self.hash.unwrap()[..],
                &self.signature.unwrap()
            )
            && self.hash.unwrap() == self.gen_hash()
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