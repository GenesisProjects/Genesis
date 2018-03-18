extern crate common;
extern crate num;

use self::common::address::Address;
use self::num::bigint::BigInt;
use self::common::hash::Hash;
use self::num::bigint::BigInt;

///
///
/// 
#[derive(Debug)]
pub struct TransactionBody {
    account_nounce: u64,
    price: BigInt,
    gas_limit: u64,
    recipient: Address,
    amount: BigInt,
    payload: vev<u8>,
    sig: Signature,
    hash: Has
}

struct Transaction {

}