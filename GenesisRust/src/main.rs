extern crate gen_network;
use gen_network::utils::*;
use gen_network::nat::*;
use std::net::SocketAddr;

fn main() {
    let test = get_local_ip();
    let result = get_public_ip_addr(Protocol::UPNP, &(SocketAddr::new(test.unwrap(), 9999), 9999));
    println!("{:?}", result)
}