extern crate common;
extern crate core;
extern crate rlp;

use self::rlp::encoder::Encoder;
use self::rlp::types::RLP;

fn main() {
    let mut encoder = Encoder::new();
    let test_item1 = RLP::RLPItem {value: "cat".to_string()};
    let test_item2 = RLP::RLPItem {value: "dog".to_string()};
    let test_list = RLP::RLPList {list: vec![test_item1, test_item2]};
    let result = encoder.encode(&test_list);
    print!("{:?}", result)
}