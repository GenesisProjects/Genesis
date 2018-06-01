extern crate gen_network;
use gen_network::utils::*;
use gen_network::nat::*;
use std::net::SocketAddr;

fn main() {
    let test = get_gateway_ip();
    println!("{:?}", test);
    let result = get_public_ip_addr(Protocol::UPNP, &(SocketAddr::new(test.unwrap(), 9999), 9999));
    println!("{:?}", result);
}