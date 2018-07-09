extern crate gen_network;
extern crate common;

use gen_network::utils::*;
use gen_network::nat::*;
use gen_network::p2p_controller::*;
use common::thread::Thread;
use std::net::*;
use std::str::FromStr;

fn main() {
    P2PController::launch_controller();
    loop {

    }
    let test = get_local_ip();
    let result = get_public_ip_addr(Protocol::UPNP, &(SocketAddr::from_str("192.168.0.1:20").unwrap(), 9999));
    println!("{:?}", test);
}