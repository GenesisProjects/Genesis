extern crate gen_network;
use gen_network::utils::*;
use gen_network::nat::*;
use std::net::*;
use std::str::FromStr;

fn main() {
    let test = get_local_ip();
    let result = get_public_ip_addr(Protocol::UPNP, &(SocketAddr::from_str("192.168.0.1:20").unwrap(), 9999));
    println!("{:?}", test)
}