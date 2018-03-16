mod apps;
mod common;
use common::key::*;
use common::address::*;

fn main() {
    let (pair, doc) =  match KeyPair::gen_rand_keypair() {
        Ok(r) => r,
        Err(e) => panic!(e.as_bytes().to_owned())
    };
    println!("Hello, world! {:?}", AddressOp::addr2string(&pair.public_key_str()));
}
