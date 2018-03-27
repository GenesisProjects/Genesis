extern crate common;
extern crate core;
extern crate rlp;

use self::rlp::encoder::Encoder;
use self::rlp::types::RLP;

fn main() {
    let mut encoder = Encoder::new();
    let test_item1 = RLP::RLPList {list: vec![]}; // []
    let test_item2 = RLP::RLPList {list: vec![test_item1.clone()]}; // [[]]
    let test_item3 = RLP::RLPList {list: vec![test_item1.clone(), test_item2.clone()]}; //  [ [], [[]] ]
    let test_list = RLP::RLPList {list: vec![test_item1.clone(), test_item2.clone(), test_item3.clone()]}; // [ [], [[]], [ [], [[]] ] ]
    let result = encoder.encode(&test_list);

    print!("{:?}", result)
}