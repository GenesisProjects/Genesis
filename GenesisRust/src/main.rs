extern crate common;
extern crate core;
extern crate rlp;

use self::rlp::encoder::Encoder;
use self::rlp::types::RLP;

fn main() {
    let mut encoder = Encoder::new();
    let test_obj = RLP::RLPItem {value: "ssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssss".to_string()};
    let result = encoder.encode(&test_obj);
    print!("{:?}", result)
}