pub mod defines;
pub mod stun;
pub mod upnp;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

pub type SocketInfo = (SocketAddr, u16);

pub enum Protocol {
    UPNP,
    STUN
}

pub fn get_public_ip_addr(protocol: Protocol, local: &SocketInfo) -> Option<SocketInfo> {
    match protocol {
        Protocol::UPNP => upnp::map_external_address_upnp(local),
        _ => panic!("Unknown protocol")
    }
}